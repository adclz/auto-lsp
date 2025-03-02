cargo bench --bench django --features=python
cargo bench --bench django_rayon --features="python rayon"
cargo bench --bench flame_graph --features=python -- --profile-time 5