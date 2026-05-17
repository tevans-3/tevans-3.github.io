use crate::blog::BlogPost;

pub fn all_posts() -> Vec<BlogPost> {
    vec![onionhead_post(), metallica_post()]
}

fn metallica_post() -> BlogPost {
    BlogPost::new(
        "Metallica Hell Yes: Handwriting RISC-V Assembly for Fun and Profit",
        "2026-05-16",
        "handwriting-riscv-assembly",
    )
    .paragraph("<figure style=\"text-align: center;\"><img src=\"https://media3.giphy.com/media/v1.Y2lkPTc5MGI3NjExeHFtbDh4Y2U5cWV0bHp2cWM4NHc2NjRkamcwOGFwZ2Mxa3N6bDNybSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/YBsd8wdchmxqg/giphy.gif\" alt=\"Making it rain\" /><figcaption><small>This is what profit looks like.</small></figcaption></figure>")
    .paragraph("<figure style=\"text-align: center;\"><iframe src=\"https://tenor.com/embed/26258581\" width=\"480\" height=\"270\" frameborder=\"0\" allowfullscreen></iframe><figcaption><small>And this is what fun looks like.</small></figcaption></figure>")
    .paragraph("You probably won't make any money from handwriting RISC-V assembly, and if you're like me and don't have a brain you won't profit by learning much either. You can't run your brain through a brain stretcher if you don't have a brain to begin with. So why even bother doing this? B-b-because--")
    .paragraph("<figure style=\"text-align: center;\"><iframe src=\"https://tenor.com/embed/12248731940596390156\" width=\"480\" height=\"270\" frameborder=\"0\" allowfullscreen></iframe><figcaption><small>Anticipate lots of headbanging (yours, on your keyboard).</small></figcaption></figure>")
    .paragraph("It's <em>cool</em>, that's why! Writing assembly by hand is fun, and instructive, and it will change your mental model of how computers work. By learning to reason at the register level, you'll gain a better, more intimate understanding of what computers are really doing when running your code. That understanding is the profit. And the fun! With that in mind, let's dive right in.")
    .paragraph("Our goal: to write a RISC-V version of John Conway's Game of Life and run it METAL on an LED matrix with a Raspberry Pi controller. The slice of Pi that we'll be using (pun, obviously, intended) is the Pi Pico H, that is, the Pico 2 with headers pre-installed. Headers are just <a href=\"https://www.pishop.ca/product/female-header-set-for-raspberry-pi-pico/\">these little spiky bits</a> which you can attach peripherals to in order to enable GPIO and debugging. If you're buying a Pico and don't own or aren't comfortable using a soldering gun, make sure you get one with the headers pre-installed-- it will save you the trouble of soldering them on yourself. (They do make solderless 'hammer-in' headers which you could buy separately and the install yourself, but it's a lot simpler to just buy one with headers ready as-is.) A Waveshare 64 x 32 LED matrix is the display we're going to target. It's <a href=\"https://www.amazon.ca/dp/B0BQYDLHY9?ref=ppx_yo2ov_dt_b_fed_asin_title&th=1\">this model</a>. You'll also need <a href=\"https://www.amazon.ca/dp/B0FND7VGQ9?ref=ppx_yo2ov_dt_b_fed_asin_title\">these</a> for powering the matrix")
    .paragraph("The RP2350, which is the actual microprocessor on the Pico 2 board, doesn't have 4 physically separate, independent cores. Instead, it has a 'redundant core architecture', where dual ISA implementations are physically present in the same positions on the die. This is different from most multicore chips, which typically give each core its own separate, physical silicon implementation. On this chip, both ISAs-- the ARM Cortex M-33 and Hazard3 RISC-V-- are physically there in the silicon, but because they share the same die positions, only one can run at any given time. Otherwise, the chip would have to manage both on shared IO and communication (serial bus) infrastructure. Which ISA gets activated is determined at boottime, when the bootrom reads a startup flag, IMAGE_DEF_BLOCK, reads the architecture that the programmer declared, and then activates that ISA on both cores.")
    .image_with_alt("../public/rp2350.png", "RP2350 microprocessor")
    .paragraph("When you first plug the Pico 2 into your computer, that bootrom is the first thing that loads. After reading IMAGE_DEF_BLOCK, reading the architecture specified there, and loading it, you're now running in machine-mode, M-mode, which is the first mode that's entered on system load or reset and operates exclusively on physical addresses. In U- (user) and S- (supervisor) modes, running code is assigned virtual addresses, which the memory management unit (MMU) translates to actual physical addresses through a process known, creatively, as Virtual Address Translation. According to the RISC-V documentation, \"M-mode is used for low-level access to a hardware platform.\" Which is great for us since that's exactly what we need.")
    .paragraph("<iframe src=\"https://tenor.com/embed/24806130\" width=\"480\" height=\"270\" frameborder=\"0\" allowfullscreen></iframe>")
    .image_with_alt("pico2-plugged-in.png", "Plugged in Pico 2 board")
    .paragraph("At this point, only core 0 is active. The only running clock is the default Ring Oscillator, or ROSC. During normal operation, a microprocessor is driven by a quartz crystal. A voltage is applied to the crystal and filtered to produce a stable signal that acts as the pulse of the chip. This works, incidentally, because of the <a href=\"https://en.wikipedia.org/wiki/Piezoelectricity\">piezoelectric effect</a> which converts electrical energy to physical vibrations. When the chip powers on, though, this external crystal isn't running, so the ROSC is needed to drive start-up. The bootrom will switch to the external crystal as soon as it detects one.")
    .paragraph("<figure style=\"text-align: center;\"><iframe src=\"https://tenor.com/embed/18524691\" width=\"480\" height=\"270\" frameborder=\"0\" allowfullscreen></iframe><figcaption><small>The modern world is powered by magical crystals.</br> Maybe you were too quick to dismiss the crystal healing people after all.</small></figcaption></figure>")
    .paragraph("There's some rigmarole involved in toolchain setup. If you feel like skipping all that, you can just clone this repo, delete the finished assembly files, and use the existing toolchain. Because we need to run our code on a separate device, there's a final flash step, where we port the machine code (an ELF file) onto the Pico. Before we can do that, we have to convert it into a bootable format, which in this case is Microsoft's uf2, or USB Flashing Format. With this, we can just drag and drop the file onto the RP2350. The flow, as you can see in the provided Makefile, is .s -> .ld -> .elf -> .uf2. The repo contains a file \"preflight checklist.md\" that lists all the toolchain dependencies you'll need to install. At this point, you should go make sure you have all the stuff in that file.")
    .paragraph("Once you've emerged from the depths of hell, you're ready to begin writing assembly. :) The first thing we need to do is define stack regions for both cores by setting stack pointers. That will involve setting a stack pointer on the first core, then syn")
}

fn onionhead_post() -> BlogPost {
    BlogPost::new(
        "Onionhead: a DIY Onion Router",
        "2026-04-01",
        "diy-onion-router",
    )
    .paragraph("This post describes how I built an onion router to run on my local machine. The system is named \"Onionhead\" in tribute to Todd Rundgren, who wrote <a href=\"https://www.youtube.com/watch?v=IzrIvDSKdS0\">a song</a> by that name.")
    .title("Preliminaries")
    .subtitle("Why Care About Onion Routing?")
    .paragraph("Cryptography has historically focused on the protection of transmitted information, paying less attention to the shielding of information metadata. This metadata itself, however, can be a significant security and privacy concern. In countries where censorship is commonplace, simply accessing certain websites may mark you as a target for persecution. \"I have nothing to hide\", you might think, \"Why should I care who knows what websites I visit?\" That attitude, however, is only viable based on your position in a privileged present. Rights erode. What is protected today may be illegal tomorrow. And even in more mundane circumstances, it's often desirable to control what you disclose about yourself and how it gets disclosed. An onion router gives you a greater degree of control. It effectively covers your tracks as you travel across a network by protecting information metadata, like what website you visited at a certain time. It prevents passive observers from linking site access requests to the individuals making those requests. That makes it broadly useful, both for things like censorship resistance and also smuggling cocaine.")    
    .paragraph("The social consequences of the technology are, thankfully, out of scope here; this post strictly focuses on 'how it works'.  To better understand that, we're going to build an onion router.")
    .colorbox_styled("#f3e5f5", "#7b1fa2", 2, "<strong>Definition 1.1.</strong> An <em>onion router</em> is a system of internetworked proxies which apply a protocol of successive encryption in order to protect information <em>metadata</em>.")
    .subtitle("Tools and Setup")
    .paragraph("Containerlab is a containerized network simulator, built on Docker. We're going to use it to configure our network topology.")
    .colorbox("#e8eaf6", "<strong>Note</strong> You need Docker to run containerlab. If you're reading this tutorial-style, this post assumes that you already have Docker installed. You can just run this Docker command to install containerlab: 'docker pull ghcr.io/srl-labs/clab'")
    .title("Configuring a Network with Containerlab")
    .paragraph("A virtual lab is a simulation of a complex network that runs on a single machine. A lab is built by defining a network topology. Where a network is the whole set of interconnected components and connections, the complete system, a network topology is an abstraction and specification of a network. It's just a set of nodes and a set of edges.")
    .colorbox_styled("#e8f5e9", "#2e7d32", 2, "Containerlab is a batteries-included, FOSS tool for building and running virtual labs. It gives you everything you need to define, run, and analyze labs out-of-the-box.")
    .paragraph("Here's an example of a network topology .yaml definition file:")
    .code_block(r"
        # topology documentation: http://containerlab.dev/lab-examples/two-srls/
        name: srl02 

        topology: 
            nodes:
                srl1: 
                    kind: nokia_srlinux
                    image: ghcr.io/nokia/nokia_srlinux
                    startup-config: srl1.cfg
                srl2: 
                    kind: nokia_srlinux 
                    image: ghcr.io/nokia/srlinux 
                    startup-config: srl2.cfg 

            links: 
                - endpoints: ['srl1:e1-1', 'srl2:e1-1']", "latex")
    .paragraph("This diagram, from the same source, may be helpful in explaining what's going on:")
    .image_with_alt("simple-lab-example.png", "Simple two-node lab example")
    .paragraph("From the above file and diagram, you can see that we literally just define a topology by naming it and then specifying the precise configuration of its nodes and links.")
    .paragraph("Onion routing systems require a minimum of three nodes to provide anonymity. To understand why, imagine you're trying to connect to some site via Tor. Your onion proxy builds a three hop circuit which hops from node1, to node2, then to the exit node, after which point you're connected with your destination application. At every step of the system, no two onion routers (or nodes) know the identity of both you and the site you want to connect to. The first node can identify your ip address, and it knows, after decrypting its layer of the onion, that your relay needs to go to node2, but it doesn't know your final destination. Node2, in turn, knows that a relay came from node1, and after decrypting its layer of the onion, that that relay needs to go to the third exit node, but it doesn't have any other information about the node's origins or its final destination. Similarly, the exit node knows the identity of the middle node and the destination, but nothing else. Now imagine there are only two nodes in between you and the site you're trying to access. If one of those nodes fails or is compromised, then a single node can now identify both you and your destination. Which means that anonymity is completely lost.")
    .paragraph("Because our system requires us to periodically switch between circuits, we need at least 4 routers (two middle, one entry and one exit).")
    .paragraph(r#"So let's write our topology file. First, we'll create a dedicated directory by running mkdir onionhead. All this tutorial's work will be done from within this directory. Next, we'll start a file named oh.clab.yml. Then we'll add the name: oh at the top. Then we define an instance of a topology and indented beneath that, the nodes. Each node's kind is nokiasrlinux, because containerlab configures TLS by default on all connections between nodes of that kind. Containerlab will automatically create a config file for each node when the lab is created, so we can leave out the "startup-config:" option. Next up are the links. Each link is defined as a pair of endpoints, which are just strings specifying which port to connect to. For example, the string "client:e1-1" means port 1 on line-card 1 on the client node. (A line card is the hardware that contains all the ports for routers and switches to connect to; in this lab, there is only one linecard, so the first part of the interface specification will always be "e1", and you don't really need to worry about or know anything about linecards). Finally, our topology file will look like this:"#)
    .code_block(r#"name: oh
topology:
  nodes:
    client:
      kind: nokia_srlinux
      image: ghcr.io/nokia/srlinux
    entry:
      kind: nokia_srlinux
      image: ghcr.io/nokia/srlinux
    middle1:
      kind: nokia_srlinux
      image: ghcr.io/nokia/srlinux
    middle2:
      kind: nokia_srlinux
      image: ghcr.io/nokia/srlinux
    exit:
      kind: nokia_srlinux
      image: ghcr.io/nokia/srlinux
    dest:
      kind: nokia_srlinux
      image: ghcr.io/nokia/srlinux

  links:
    - endpoints: ["client:e1-1", "entry:e1-4"]
    - endpoints: ["entry:e1-1", "middle1:e1-4"]
    - endpoints: ["middle1: e1-1", "exit:e1-4"]
    - endpoints: ["exit:e1-1", "dest:e1-3"]
    - endpoints: ["middle2:e1-1", "exit:e1-3"]
    - endpoints: ["entry:e1-2", "middle2:e1-4"]"#, "yaml")
    .paragraph("We'll need to add some more configuration options to our nodes eventually, but for now this is enough. Next, just to make sure containerlab and our topology are set up properly, let's deploy our network. We'll run this command to start a containerlab shell:")
    .code_block(r#"docker run --rm -it --privileged \
    --network host \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v /var/run/netns:/var/run/netns \
    -v /etc/hosts:/etc/hosts \
    -v /var/lib/docker/containers:/var/lib/docker/containers \
    --pid="host" \
    -v $(pwd):$(pwd) \
    -w $(pwd)
    ghcr.io/srl-labs/clab bash"#, "bash")
    .paragraph("Once up and running, the shell should look like this: <code>username:/path/to/cwd#</code>")
    .paragraph("Now we can run: <code>containerlab deploy</code>")
    .paragraph("You should see a number of INFO messages which inform you that containerlab is pulling images and creating containers (one container for each node defined in the network topology file) and links, like this:")
    .image_with_alt("../public/deployed-lab.png", "Deployed containerlab")
    .paragraph("As you can see, each node is running, and containerlab has assigned each node its own IPv4/6 address.")
    .paragraph("Now we can run the command <code>containerlab destroy</code> to tear down the lab, as it can consume quite a bit of system resources (mostly during start-up) and we won't be using it for a while.")
    .paragraph(r#"We can also get a graphic representation of the topology graph at any time by running the command: <code>graph -t oh.clab.yml</code> and then pointing our browser to one of the listed addresses, where we'll see that containerlab is serving us the graph. If you've defined your topology file correctly, it should look like this (you may have to drag the nodes around a bit to get it looking exactly like this):"#)
    .image_with_alt("../public/graph-topology.png", "Graph topology")
    .paragraph(r#"We'll exit the containerlab shell just by entering "exit". The next step is to build the actual components of the onion routing system."#)
    .title("Building the Client")
    .paragraph("The client is the user-side application that connects a user to the onion routing system. The client is responsible for circuit construction, sending/receiving relay cells and opening/closing streams.")
    .subtitle("Circuit Construction")
    .paragraph("In an onion routing system, all data is transmitted in fixed size cells. These cells follow a standard format which is defined in the system's specification. In this system, all cells will follow the TOR format. This means that all cells are 514 bytes long, consist of a header and a payload, and are one of two types: relay (data-carrying) or control (command-carrying, used to instruct some downstream node to take a specific action).")
    .paragraph("A circuit is a sequence of nodes which defines the path a client's cell travels to reach its destination. In order to construct a circuit, the client negotiates key exchanges with each node it intends to include in the circuit. It then takes its message and destination and encrypts that using the exit node's key. It concatenates a relay cell to the end of this ciphertext; the concatenated relay cell tells the middle node to send the cell to the exit node. The ciphertext and the concatenated relay cell are then encrypted using the middle node's key. This exact same process is repeated for the entry node (or in circuits more than three-hops long, for every node between the middle and entry nodes). (This layering of encryption is the reason it's called onion routing; because, like onions, the encrypted cells have many layers). This diagram shows what an onion looks like after all these encryption operations have been performed:")
    .image_with_alt("../public/Onion_diagram.png", "A fully encrypted onion")
    .paragraph("When a node receives a relay cell, it decrypts it using its private key. The decrypted payload will just be the concatenated relay cell which tells the node where to route the cell next; the actual message is still wrapped under several layers of encryption. The node then forwards the cell along, and similarly, at every step of the way, each node unwraps another layer of the onion, until finally, at the exit node, the entire message is decrypted.")
    .paragraph("We'll need to implement the following methods: <ol><li>A method of establishing private keys for all the nodes</li><li>A method of establishing public key parameters (onion keys) for all the nodes</li><li>A method of initiating public key handshakes between nodes</li><li>A method of hashing keys to verify that the intended onion router actually completed the handshake</li></ol>")
    .subtitle("Key Generation and Management")
    .paragraph(r#"Every relay node will have two public-private keypairs: (1), an identity keypair and (2), an onion keypair (in the TOR documentation, (2) is referred to as a "circuit extension" keypair). The identity keypair lasts the lifetime of the node, whereas the onion keypair has a shorter lifetime. The ephemerality of the onion key provides an important security benefit in that compromised keys are guaranteed to be useless after a fixed amount of time. The onion keys are used to encrypt and decrypt data during circuit construction, which is why it makes sense to rotate them out periodically."#)
    .paragraph("The client will maintain a look-up table of key value pairs where each key is a node's node id and each value is that node's public key. When a node updates its keypair, it will need to notify the client so the client's look-up table can be updated with the node's new public key.")
    .paragraph("We will use OpenSSL's implementation of RSA to handle the actual key generation.")
    .paragraph("We'll implement the following methods:")
    .code_block(r#"GenerateRelayIdentityKeypair(string nodeId, string dirAddress);
GenerateRelayOnionKeypair(string nodeId, string dirAddress);
GetRelayOnionKeyLifespan(string nodeId);
RefreshRelayOnionKey(strihg nodeId, string dirAddress);
SendNewPubKeyToClient(string dirAddress, string key, string type);"#, "cpp")
    .paragraph(r#"For our client, we'll need a method to establish a shared secret key with each handshake. Each relay will need a method to return the handshake. The returned handshake will include the value \(g^y\), where g is the agreed-upon public key parameter and \(y\) is the relay's private key, as well as a hash \(H(K|\text{"handshake"})\), where \(K=g^{xy}\). This hash allows the client to verify that a key has been established with the correct router. To understand why this works, consider how the client will validate the relay's returned handshake. The client has the relay's public key, the value \(g^y\), and its private key \(x\). It can compute \(g^{xy}\) and then compute a hash of that value \(| \text{ handshake}\). Because hash algorithms are deterministic, if the node that it exchanged \(g^x\) with is also the node that returned \(g^{xy}\), then the hashed value \(H(K|\text{"handshake"})\) will equal the hash returned by the relay."#)
    .code_block(r#"OfferHandshake(string nodeId, string myHalfOfKey);
ReturnHandshake(string nodeId, string myHalfOfKey, string hashedVal);
AuthenticateHandshake(string key, string hash);"#, "cpp")

}
