pub fn is_sane(name: &str, _: &()) -> garde::Result {
    if name.chars().all(|c| !c.is_control()) {
        Ok(())
    } else {
        Err(garde::Error::new("contains invalid control characters"))
    }
}