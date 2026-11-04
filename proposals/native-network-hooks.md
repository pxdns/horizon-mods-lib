# Proposal: transport and packet hook points for Horizon native mods (`NATIVE_HOOKS_API` 5)

Status: draft, for the Horizon maintainers. Written against Horizon `ba6344a` (`NATIVE_HOOKS_API` 3, protocol 777 = 26.3) while
looking at what it would take for a native mod to (a) join servers that run an older version, such as 1.21.11, the way ViaVersion
and ViaBackwards let a newer client do, and (b) connect over a WebSocket, as Eaglercraft servers need. Nothing here is implemented.
It is the companion of [the screen, slot and map proposal](native-screen-hooks.md); numbering assumes that one lands first (API 4).

## What blocks it today

Nothing in the hook-point contract touches the connection. Reading `crates/horizon-protocol` and `crates/horizon-client/src/net.rs`:

- **The version is a constant.** `PROTOCOL_VERSION` (777, `horizon-core/src/version.rs`) goes into the handshake's `Intention` at
  `net.rs:366`, `:438` and `:834` (play, and the two status pings). A mod cannot say "this server speaks 1.21.11".
- **The transport is `TcpStream`.** `Connection::connect_server` resolves and connects a `TcpStream` (`connection.rs:216`,
  `from_stream` at `:269`); the only other transport is the in-memory `Pipe` an integrated server uses. There is no place to
  supply another one, so no WebSocket.
- **Packets are typed.** `net::run` decodes frames into `play::Clientbound` / `Serverbound` enums for protocol 777 and the ids in
  `ids.rs` are generated for 777 only. A packet of another version is a decode error (`Event::DecodeError`, skipped) or a
  mis-decode. The raw `(id, body)` pair exists, in `ConnReader::recv` and `ConnWriter::send`, but only inside the crate.
- **Mods cannot add dependencies.** horizon-mixins "adds no Cargo dependencies", so a mod cannot bring a WebSocket client.
  Horizon already carries `rustls`, `sha1`, `flate2` and `ureq` in its `Cargo.lock`, but no WebSocket crate.

## Design

Three additions, usable alone. Same rules as before: added only, empty default bodies, no `cfg` or macros in the bodies, one
check when nothing uses them, `"hooks": 5` in the bundle.

### 1. The protocol a server is spoken to in

```rust
/// The protocol number to put in the handshake for `address` (host:port as typed), or none for Horizon's own.
fn native_protocol(&self, address: &str) -> Option<i32>;
```

Used by all three handshake sites. With none, nothing changes. When it returns a number, Horizon must not assume the server
speaks 777 afterwards, so the typed decoders are switched off for that connection (the next hook point takes over), and the
connecting screen shows the server's version string from the status ping (`status_response` is JSON text and is not
version-specific, so it needs no translation beyond the handshake number).

### 2. Packet translation, on the network thread

Horizon decrypts and decompresses; the mod sees plain frames and returns zero or more replacement frames in the **other side's**
format. Mods must not receive the game, because the network thread has none; a translator is an object the game thread builds
once per connection and hands over:

```rust
pub trait NativeTranslator: Send {
    /// The server's protocol number this translates from.
    fn server_protocol(&self) -> i32;
    /// A frame from the server (`id`, `body` without the length or compression framing) in `state`:
    /// push what Horizon (26.3) should see instead, in order. Unchanged frames are pushed as `Frame::Same`.
    fn clientbound(&mut self, state: NativeState, id: i32, body: &[u8], out: &mut Vec<Frame>) -> Result<(), String>;
    /// A frame Horizon sent: push what the server should receive.
    fn serverbound(&mut self, state: NativeState, id: i32, body: &[u8], out: &mut Vec<Frame>) -> Result<(), String>;
}
pub enum Frame { Same, Replace { id: i32, body: Vec<u8> } }
pub enum NativeState { Handshake, Status, Login, Configuration, Play }

/// Offer a translator for `address`; the first mod to push one wins.
fn native_translator(&self, address: &str, protocol: i32, out: &mut Option<Box<dyn NativeTranslator>>);
```

Points that matter for correctness and speed:

- **State changes.** The translator is told the state of every frame. The packets that switch it (`login_finished`,
  `finish_configuration`, `start_configuration`, `login_acknowledged`) are known by id per version, so a translator can follow them;
  Horizon keeps its own view from the translated frames, so both ends agree.
- **Encryption and compression stay in Horizon.** `enable_encryption` and `set_compression` act on the real stream; the
  translator never sees the shared secret, the session token or the RSA key exchange, only frames after decryption. Compression
  threshold packets are passed to Horizon unchanged and a translator is told the threshold only for information.
- **No extra cost on the common path.** `Frame::Same` copies nothing: Horizon forwards the original buffer. Most packets are
  identical between neighbouring versions, so the average cost of a translator is one table lookup per packet. A budget is part of
  the contract: the hook may not block, and Horizon logs a connection whose translation takes more than 2 ms for a frame.
- **Runs off the game thread.** `net::run` already owns the connection on its own thread (`net.rs:397`); the translator lives
  there, so frame rate and tick time do not see it. The one cross-thread copy is the existing event channel.
- **Failure is a disconnect, not a crash.** An `Err` from the translator ends the connection with its text as the reason.

### 3. A transport a mod, or Horizon, can supply

```rust
pub trait NativeTransport: Read + Write + Send {
    fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()>;
    fn shutdown(&self);
    fn try_clone(&self) -> io::Result<Box<dyn NativeTransport>>;
}
/// Open a connection to `address` instead of Horizon's TCP; the first mod to answer wins.
fn native_connect(&self, address: &str, timeout: Duration, out: &mut Option<Box<dyn NativeTransport>>);
```

`connection.rs`'s `Stream` enum (TCP or `Pipe`) gains a third case holding the boxed transport; framing, compression and
encryption run on top of it unchanged, so a transport only moves bytes. Addresses keep working as typed; a scheme
(`ws://host`, `wss://host/path`) is how a mod claims one.

Because mods cannot add crates, WebSocket needs one of two choices:

- **A.** Horizon gains a small RFC 6455 client (`ws`/`wss`, binary messages) built on the `rustls` and `sha1` it already has, and
  registers it as a built-in transport for those schemes. A mod then only needs hook 1 and 2.
- **B.** The launcher lets a bundle declare crates.io dependencies it needs, pinned in its `horizon-native.json`, which the build
  already resolves from crates.io. This helps every future mod, but it widens what a "trust native code" build pulls in.

I would take A for the WebSocket and leave B as a separate discussion.

## What these unlock, and what they do not

**Joining older servers (the ViaVersion idea).** A translator mod, `vialike-1-21-11`, would implement `NativeTranslator` for
1.21.11 → 26.3. How much work that is depends on how far the two packet and data sets differ, which can be measured rather than
guessed: the data generator already emits a `packets` report per version (the source of `ids.rs`). The first deliverable should be
a mechanical diff of those reports (packet ids per state, field layouts) plus the block, item, entity-type and registry id tables,
shipped as the mod's data files (`native-data/<mod-id>/`), and only then an estimate. The usual hard parts, from experience of
what Via-style projects spend their time on, are:

- **Configuration-state registries and tags**, which change between versions and which Horizon needs complete.
- **Chunk data**: palette and heightmap formats, light data, block-state id remapping inside chunk sections. This is the heavy
  packet and the one where "no lag difference" is earned or lost; remapping palettes in place, not decoding blocks, is how.
- **Items**: data components against NBT, item ids, recipe and advancement formats, `container_click` slot changes. This is where
  "the proper items from before" lives.
- **Entity metadata** serializer ids and entity type ids.
- **Behaviour the server expects of the old client**, for anti-cheat plugins: movement and interaction packets must look like
  the older client's.

A ladder of one-step translators (1.21.11→26.1→26.2→26.3, each tested against the previous) is easier to verify than one big
jump, and each step can be released as its own mod. Going back much further (1.8, 1.12) is a long ladder; it is the same
design, not a different one, and I would not promise it up front.

**Eaglercraft.** The WebSocket transport (hook 3, choice A) covers the carrier. Eaglercraft servers also have their own opening
exchange before the Minecraft protocol starts, and speak 1.8 or 1.12 protocol, so a working connection needs both that
exchange (a `native_translator` that owns the `Handshake` and `Login` states) and the long ladder above. I have not read the
Eaglercraft protocol for this proposal; it needs a spec check before anyone commits to it. Treat Eaglercraft as the WebSocket
transport plus a research item.

**Not covered.** How the browser build reaches servers at all (the offline page ships a launcher with relays, and TCP is not
available there) was not read for this proposal; the transport hook must work in that build too, which probably means the
WebSocket transport is the *only* one there, and is worth checking first.

## Licensing and trust

- ViaVersion, ViaBackwards and ViaRewind are GPL-3.0. horizon-mods-lib is MIT, so a translator here has to be an independent
  implementation from the protocol data (the generator's reports and Mojang's published formats), not a port of their code. State
  that in each translator's README, as the other mods do.
- A translator sees every frame of a play session. It is native code, so it is already inside the "trust native code" boundary,
  but the hook's documentation should say that plainly, and that the session token and encryption keys are not passed to it.
- Hooks 1 and 3 let a mod choose where a connection goes. The launcher's trust prompt for native builds should list "can change
  how and where the game connects" among what a bundle may do, so a player is not surprised.

## Suggested order

1. `native_protocol` and the raw frame pass-through with `native_translator` (no new transport). A first translator can be tested
   against a real 1.21.11 server with only these.
2. Measure (first pass done: [774 against 777](measurements/774-vs-777.md)): the 774 -> 775 step is small and a translator's own
   per-packet cost is negligible; the exact 774 -> 777 id table still needs Mojang's generator reports.
3. `NativeTransport` and the built-in `ws`/`wss` client (choice A).
4. Eaglercraft, as research then a translator.

## Open questions

- Should `NativeTranslator` see frames before or after Horizon's own cookie and resource-pack handling in configuration?
  (Before is simpler for a translator; after keeps those features version-independent.)
- Does the integrated server's `Pipe` need the hooks, or only remote connections? (Only remote, I suggest.)
- Where do translator data tables live so they are not 20 MB inside a `.hnmod`: `native-data/<mod-id>/` as now, or fetched?
- Is a per-frame time budget enforceable on the network thread without cost, or only measured?
