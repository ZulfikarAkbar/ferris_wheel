// FIXME: Make me pass! Diff budget: 25 lines.

enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

fn to_ms(d: Duration) -> u64{
    let mut ms: u64;    
    match d{
        Duration::MilliSeconds(x) => {ms = x},
        Duration::Seconds(x) => {ms = x as u64 * 1000},
        Duration::Minutes(x) => {ms = x as u64 * 60 * 1000}
    }
    
    ms
}

fn main() {
    assert_eq!(to_ms(Duration::Seconds(120)), to_ms(Duration::Minutes(2)));
    assert_eq!(to_ms(Duration::Seconds(420)), to_ms(Duration::Minutes(7)));
    assert_eq!(to_ms(Duration::MilliSeconds(420000)), to_ms(Duration::Minutes(7)));
    assert_eq!(to_ms(Duration::MilliSeconds(43000)), to_ms(Duration::Seconds(43)));
}
