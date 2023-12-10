use std::string;

const URL: &str = "p.imt.re:80";
pub mod crash;
pub mod diag;
pub mod fond;

// sudo mount -t tmpfs -o size=512M tmpfs /mnt/mytmpfs
// sudo mkdir /mnt/mytmpfs
// ffmpeg -video_size 1920x1080 -framerate 25 -f x11grab -i :0.0 -f image2 -vf scale="1024:-1" -update 1 /mnt/mytmpfs/img.jpg





// ffmpeg -video_size 1920x1080 -framerate 25 -f x11grab -i :0.0 -f image2 -vf scale="1024:-1" -update 1 img.jpg