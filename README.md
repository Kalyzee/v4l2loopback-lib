# V4L2Loopback Rust Lib

You need the latest version of V4L2LOOPBACK module

```
git clone https://github.com/umlaeute/v4l2loopback
make 
sudo make install
```

Add a new device : 
```
        let mut loopback = V4L2Loopback::new();
        let result = loopback.add(50);
```