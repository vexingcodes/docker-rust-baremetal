# docker-rust-baremetal
A Dockerized enviroment for following along with Philipp Oppermann's [Writing an OS in Rust](https://os.phil-opp.com/).

The Rust code in this repository was taken from the [post-02](https://github.com/phil-opp/blog_os/tree/post-02) branch
of the [phil-opp/blog_os](https://github.com/phil-opp/blog_os/) repository which has the code from the blog under the
MIT license. It simply prints a Cyan "Hello World!" to the screen in VGA text mode.

# Prerequisites
These programs must be installed on the host operating system. All other software is managed within the Docker container
itself:

* Docker
* VNC Viewer

# Usage
Simply build the Docker image as usual, e.g. `docker build . --tag docker-rust-baremetal`. All code in the current
directory will be copied to `/src` in the image as part of the build process. Once the image is built, it can be run
without any special parameters, e.g. `docker run -it --rm docker-rust-baremetal`. By default, running the container will
drop you into a bash shell in the `/src` directory. Once in the container, the code can be run in QEMU using the
`cargo xrun` command. This will launch a VNC server running on port `5900` in the container. From your host you can
connect to that VNC server using a VNC viewer given the correct IP address. To find the IP address you can use the
`ip addr show dev eth0` command. For example, if the container's IP address is `172.17.0.2`, and your VNC viewer is
`gvncviewer` then you can connect to the VNC server using the command `gvncviewer 172.17.0.2`.

To be able to modify the code outside of the container and rebuild it inside the container you can mount the repository
directory as a volume into the container, e.g. `docker run -it --rm --volume "$(pwd):/src:rw" docker-rust-baremetal`.
