```rust

pub type SendableFn = Arc<Mutex<(Fn(&str, &str) + Send + Sync + 'static)>>;

#[derive(Clone)]
pub struct MqttConnectionOptions {
    pub id: String,
    pub keep_alive: u16,
    pub clean_session: bool,
}

pub struct MqttConnection {
    pub stream: Option<TcpStream>,
    pub current_pkid: AtomicUsize,
    pub queue: LinkedList<PublishMessage>,
    pub length: u16,
    pub retry_time: u16,
}

#[derive(Clone)]
pub struct MqttClient {
    pub options: MqttConnectionOptions,
    pub connection: Arc<Mutex<MqttConnection>>,
    pub msg_callback: Option<Sender<SendableFn>>,
}

impl MqttClient{
    pub fn publish(&mut self, topic: &str, message: &str, qos: QualityOfService) {

        let mut gconnection = self.connection.lock().unwrap(); //Got mutex guard
        let ref mut publish_queue = gconnection.queue; 
        // Getting refs of actual 'connection' members using mutex guard
        // is borrowing entire mutex guard to RHS and hence below        
        // statements are'nt valid
        let ref mut current_pkid = gconnection.current_pkid;
        let ref mut stream = gconnection.stream;
    }
}

ERROR:
-----

error: cannot borrow `gconnection` as mutable more than once at a time [E0499]
src/client/publish.rs:20         let ref mut current_pkid = connection.current_pkid;
                                                            ^~~~~~~~~~
src/client/publish.rs:20:36: 20:46 help: run `rustc --explain E0499` to see a detailed explanation
src/client/publish.rs:19:37: 19:47 note: previous borrow of `connection` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `connection` until the borrow ends
src/client/publish.rs:19         let ref mut publish_queue = connection.queue;
```
———————————————————————————————————————

WORKS
-----

```rust
pub fn publish(&mut self, topic: &str, message: &str, qos: QualityOfService) {

        //reference to actual `connection` after defering mutex guard
        let ref mut connection = *self.connection.lock().unwrap(); 
        //creating refs to underlying members using actual 'connection'        //reference doesn't seem to be a problem
        let ref mut publish_queue = connection.queue; 
        let ref mut current_pkid = connection.current_pkid;
        let ref mut stream = connection.stream;
```
DOESN't WORK
------------

```rust
impl MqttClient{
    pub fn publish(&mut self, topic: &str, message: &str, qos: QualityOfService) {

        let mut connection = *self.connection.lock().unwrap();
    }
}

error: cannot move out of borrowed content [E0507]
let mut connection = *self.connection.lock().unwrap();
```

&mut self is a reference. You cannot move its (actual structs) underlying content using a reference

WORKS
—————

```rust
impl MqttClient{
    pub fn publish(&mut self, topic: &str, message: &str, qos: QualityOfService) {

        let mut gconnection = self.connection.lock().unwrap();
        let MqttConnection{ref mut stream,
             ref mut current_pkid,
             ref mut queue,
             ref mut length,
             ref mut retry_time} = *gconnection; 
             // dereferencing mutex guard (gets actual connectin struct)
             // and creating reference to each struct member
             // Since it's &mut self and you are not moving any underlying
             // content of the (self)struct, it's not a problem
    }
}
```


WORKS
—————

```rust
impl MqttClient{
    pub fn publish(&mut self, topic: &str, message: &str, qos: QualityOfService) {

        let mut connection = &mut *self.connection.lock().unwrap();
        let ref mut publish_queue = connection.queue;
        let ref mut current_pkid = connection.current_pkid;
        let ref mut stream = connection.stream;
    }
}
```


Usually you can separately borrow different fields of a struct, but in this case I think it's failing because what you actually have is a MutexGuard<MqttConnection> and each of your borrows needs to call `deref_mut` on the MutexGuard and that `deref_mut` borrows the whole MutexGuard

this seems to be one of the cases where Rust keeps the temporary MutexGuard alive long enough
——————————————————

I'm accessing MutexGuard through &self (not self) and hence I couldn't do *connection (struct borrowed & moving underlying content not allowed) but when I do &*connection, I'm derefering mutex guard and creating normal reference (original value is now safe because this is again a ref) --> Is this right

```rust
let mut connection = *self.connection.lock().unwrap();

error: cannot move out of borrowed content [E0507]
let mut connection = *self.connection.lock().unwrap();
```

Example:
———————

```rust
struct A {
    s: String,
    n: i32,
}

fn main() {
    let mut a = A{s:"hello".to_string(), n:10};
    
    let ra = &mut a;
    
    let b = ra.s; //trying to move 's' through a reference. 
    // If 'b' allocates directly at this stage, Reallocation might happen 
    // and internal reference changes and a.s becomes invalidated

    //let ref b = ra.s --> Now 'b' is just one more reference to actual 's' 
    //                     and when reallocation occurs, 's' s internal refernce
    //                     get updated instead of b's as in above case
}
```






