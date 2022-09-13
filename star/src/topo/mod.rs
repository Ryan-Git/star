use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::process::Command;
use std::str;
use std::{fs, io};

#[derive(Eq, Copy)]
struct Socket {
    id: i32,
}

#[derive(Eq, Copy)]
struct Core {
    id: i32,
    socket_id: i32,
    node_id: i32,
}

#[derive(Eq, Copy)]
struct CPU {
    id: i32,
    core_id: i32,
    socket_id: i32,
    node_id: i32,
}

#[derive(Eq)]
struct Node {
    id: i32,
    size: u64,
    cpus: BTreeSet<i32>,
}

#[derive(Default)]
struct Topology {
    cpus: BTreeMap<i32, CPU>,
    cores: BTreeMap<i32, Core>,
    sockets: BTreeMap<i32, Socket>,
    nodes: BTreeMap<i32, Node>,
}

impl Topology {
    pub fn cpu_info() -> Vec<CPU> {
        let output = Command::new("lscpu")
            .arg("-e=CPU,CORE,SOCKET,NODE")
            .output()
            .expect("`lscpu -e` error");

        assert!(output.status.success());

        let body = str::from_utf8(&output.stdout)
            .expect("output should be utf8")
            .split('\n')
            .collect();

        let mut cpus = vec![];

        let it = body.iter().skip(1);

        for &line in it {
            let mut line_it = line.split_whitespace();

            cpus.push(CPU {
                id: line_it.next().unwrap().parse().unwrap(),
                core_id: line_it.next().unwrap().parse().unwrap(),
                socket_id: line_it.next().unwrap().parse().unwrap(),
                node_id: line_it.next().unwrap().parse().unwrap(),
            });
        }

        cpus
    }
    pub fn numa_info() -> io::Result<Vec<Node>> {
        let mut ret;

        for entry in fs::read_dir("/sys/devices/system/node")? {
            let entry = entry?;
            if entry.file_name().
        }

        ret
    }

    pub fn new(cpu_info: Vec<CPU>) -> Topology {
        let mut topo = Topology::default();

        for cpu in cpu_info {
            topo.cores.insert(
                cpu.core_id,
                Core {
                    id: cpu.core_id,
                    socket_id: cpu.socket_id,
                    node_id: cpu.node_id,
                },
            );
            topo.sockets
                .insert(cpu.socket_id, Socket { id: cpu.socket_id });
            topo.nodes.insert(
                cpu.node_id,
                Node {
                    id: cpu.node_id,
                    size: 0,
                    cpus: Default::default(),
                },
            );

            topo.cpus.insert(cpu.id, cpu);
        }

        t
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
