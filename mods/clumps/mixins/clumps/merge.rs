// ExperienceOrb.scanForMerges, Clumps' way: any orbs that touch become one orb worth all of them.
fn clumps_merge(game: &mut crate::game::Game<'_>) -> std::ops::ControlFlow<()> {
    use horizon_client::entities::lib_clumps as m;
    let orbs = &mut game.orbs;
    if !m::on() {
        return std::ops::ControlFlow::Continue(());
    }
    let ticks: Vec<u32> = orbs.iter().map(|o| o.ticks).collect();
    let areas: Vec<_> = orbs.iter().map(|o| o.bounds().inflate(Vec3::new(0.5, 0.5, 0.5))).collect();
    let boxes: Vec<_> = orbs.iter().map(ExperienceOrb::bounds).collect();
    let plan = m::plan(&ticks, |i, j| boxes[j].intersects(areas[i].clone()));
    let mut gone = vec![false; orbs.len()];
    for (i, taken) in plan {
        let mut total = m::worth(orbs[i].value, orbs[i].count);
        let mut age = orbs[i].age;
        for j in taken {
            total = total.saturating_add(m::worth(orbs[j].value, orbs[j].count));
            age = age.min(orbs[j].age);
            gone[j] = true;
        }
        orbs[i].value = total;
        orbs[i].count = 1;
        orbs[i].age = age;
    }
    let mut gone = gone.into_iter();
    orbs.retain(|_| !gone.next().unwrap_or(false));
    std::ops::ControlFlow::Break(())
}
