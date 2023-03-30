# Value 

_**Value**_ attribute is used in enum field. It takes value of the EdgeDB enum variant.

    #[value()]
    

### Usage 

```rust
    enum Gender {
        #[value("man")] Male,
        #[value("woman")] Female
    }
```