use unsizing_experiments::CoerceUnsized;

fn main() {
    let _: std::sync::Arc<[_]> = std::sync::Arc::new(std::vec![[0; 10]; 10]).coerce_unsized();
}
