use perq::oeis;
use perq::ModIntP32;

fn main() {
    let db = oeis::ShortSeqDB::<ModIntP32>::from_stripped("stripped".to_string()).unwrap();
    db.connectivity().unwrap();
}
