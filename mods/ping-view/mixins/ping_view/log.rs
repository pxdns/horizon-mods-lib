// SampleLogger.logSample: keep a copy when this is the ping logger.
fn ping_view_log(logger: &SampleLogger, value: i64) {
    lib_ping_view::sample(logger as *const SampleLogger as usize, value);
}
