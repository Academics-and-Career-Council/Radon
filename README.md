# Radon

Backend for Resources portal for AnC.

Set up the server locally

```bash
$ git clone git@github.com:Academics-and-Career-Council/Radon.git
$ cd Radon
$ rustup override set nightly
$ cargo run -- --cors=true # for enabling CORS
$ cargo run # to run withour CORS
```

env Variables
```MONGO_URL=mongodb+srv://anc:resources2021@cluster0.ypmnq.mongodb.net/development?retryWrites=true&w=majority
DATABASE=production20```
