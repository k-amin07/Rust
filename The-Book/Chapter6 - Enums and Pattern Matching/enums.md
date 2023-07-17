Enums provide a way of defining the possible values for a variable. We might want to say that `Rectangle` is one of the possible `Shapes` that also include `Circle` and `Triangle`.
IP addresses for instance, can either be v4 or v6 but not both. They are fundamentally the same type and should both be treated as IP addresses. We can handle this by defining `IpAddrKind` enum as follows

```
enum IpAddrKind {
    v4,
    v6
}
```

`IpAddrKind` is now a custom datatype that can be used elsewhere in our code. We can create instances of this enum like
```
    let v4 = IpAddrKind::v4;
    let v6 = IpAddrKind::v6;
```

We can also create a function that takes `IpAddrKind` as argument
```
fn route(ip_kind: IpAddrKind) {}
```

and use it with either v4 or v6 type
```
    route(IpAddrKind::v4);
    route(IpAddrKind::v6);
```

We can also define the type of the enum data and use it directly, rather than using it with structs.

```
enum IpAddr {
    v4(String),
    v6(String)
}

let home = IpAddr::v4(String::from("127.0.0.1"));
let loopback = IpAddr::v6(String::from("::1"));
```

Unlike structs, name of each enum variant also becomes its constructor, i.e. `IpAddr::v4()` is a function that takes a String argument and returns an instance of type `IpAddr`.

Another advantage of using enums rather than structs is that each variant can have different types and amounts of data associated with it.

```
    enum IpAddr {
        v4(u8,u8,u8,u8),
        v6(String)
    }
    let home = IpAddr::v4(127,0,0,1);
    let loopback = IpAddr::v6(String::from("::1"));
```

The standard library has a definition for IP addresses that looks like this
```
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}
```

This shows that we can use any type of data within an enum. Note that even though the standard library contains a definition for `IpAddr`, we can still create and use our own definition without conflict because we haven’t brought the standard library’s definition into our scope. We’ll talk more about bringing types into scope in Chapter 7.

Lets look at another example enum
```
enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
```

Here, `Quit` has no data associated to it. `Move` has named fields like a struct does, `Write` includes a single `String` and `ChangeColor` includes three i32 values. In terms of structs, we'd have to define four different structs.

```
struct QuitMessage; // Unit Struct
struct MoveMessage {
    x:i32,
    y:i32
};
struct WriteMessage(String); // tuple struct
struct ChangeMessageColor(i32,i32,i32); //tuple struct
```
Writing a function that accepts these four different structs would is not easy, especially compared to `Message` enum.
We can define methdos on the `Message` enum like this
```
impl Message {
    fn call(&self) {
        // some logic here
    }
}
let message = Message::Write(String::from("hello"));
message.call();
```