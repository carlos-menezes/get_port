# get-port
> Get an available port

---

## Usage

#### 1. Get the first available port in range 1024 <-> 65535 (default range)
```rust
use get_port;

fn main() {
    let port = get_port::get_port().unwrap();
}
```

#### 2. Get a port in a specific range

```rust
use get_port;

fn main() {
    let port = get_port::get_port_in_range(get_port::PortRange { min: 5000, max: 6000 }).unwrap();
}
```

#### 3. Get a specific port if available
**NOTE:** returns the first one or falling back to an available port in range 1024 <-> 65535

```rust
use get_port;

fn main() {
    let port = get_port::get_port_prefer(vec![20, 60, 6943]).unwrap(); // Will return 6943 if available, as 0 <-> 1024 are system ports.
}
```

#### 4. Exclude a list of ports and get the first available
```rust
use get_port;

fn main() {
    let port = get_port_except(vec!(1024, 1025, 1026)).unwrap(); // Will return 1027 if available, as 0 <-> 1024 are system ports.
}
```

#### 4. Exclude a list of ports and get the first available in a specific range
```rust
use get_port;

fn main() {
    let range = PortRange { min: 5000, max: 7000 };
    let p = get_port_in_range_except(range, vec!(5000)).unwrap(); // Will return 5001 if available.
}
```
