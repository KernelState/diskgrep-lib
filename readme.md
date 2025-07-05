### The diskgrep library usage guide

sorry but the current version is trash a lil it was made at 3:30 am i wanted to sleep lol

# Installation
currently there is no library for diskgrep on crates.io so just git clone it to the root dir of your project
So your root dir should look like this:
```
-- project_name
----- src
----- cargo.toml
----- cargo.lock
----- diskgrep
```

# Usage
so the library splits into:
- disk management (mounting, unmounting and listing)
- identification (tags and filters)
disk management is in the `diskgrep::disk` sub crate which has four crates that you care about `diskgrep::disk::mount`, `diskgrep::disk::scanner`, `diskgrep::disk::units` and `diskgrep::disk::model`
for the model sub crate you have a `Disk` object and a `Partition` Object and they have the following fields (btw the app does not use private fields, which means that the data in an object is fully shown):
- `pub struct Partition {
    pub name: String,
    pub size: Size,
    pub fstype: String,
    pub uuid: String,
    pub mountpoint: Option<String>,
    pub mounted: bool,
}`
  the name is just the name after /dev/ not the full name, the Size is available in `units.rs`, fstype is just a plain string same with uuid, mountpoint is optional but as long as its mounted its guaranteed that it will be `Option::Some(v)`
- `pub struct Disk {
    pub name: String,
    pub serial: String,
    pub model: String,
    pub size: Size,
    pub children: Vec<Partition>,
}`
  same things as above from the partition except for the children its just a `Vec` of partitions

now the mounting algorithm will require structs provided by `diskgrep::disk::model` as the lib uses a C-style sub-function
- `pub fn mount(partition: &mut Partition, rw: bool, remount_ro: bool, defaults: bool, dir_name_: String) -> Result<(), diskgrep::utils::error::Error>`
`rw` is read and write, `remount_ro` is the fail-safe of remounting in Read-Only mode, `defaults` disregards all the ones above using linux's default mounting settings (you basically dont know what your doing and your praying that it works) and `dir_name_` is the mount dir which can be disregarded by using `String::new()`

- `pub fn umount(partition: &mut Partition) -> Result<(), Error>`
it explains it self

and now with `diskgrep::identification::identification`:
- `pub fn find(Id: &mut Id, debug: &bool)`
it gets an `Id` which originates from `identification::model::Id` and finds all candidates then appends them to the candidates field from `Id`

`diskgrep::identification::tag`:
- `pub fn save(name: String, dir_path: Option<String>, id: Id) -> Result<(), Error>`
`name` is the name of the file, `dir_path` is an optional directory to save the file at and `id` is an `Id` that should be saved

- `pub fn read(file_path: String) -> Result<Id, Error>`
`file_path` is the path of the file and it never defaults unlike `save`

