# Pdec

Pdec is an small application to visualize connection times of players as pulled from [https://github.com/MarechJ/hll_rcon_tool](https://github.com/MarechJ/hll_rcon_tool).
It is a very hacky project with no a lot of safety in mind.
It breaks easily and assumes users only put in correct data.

# Running

## Compile Locally

The most safe way is to compile the project locally and run it there too. 
This requires you to have installed:

- (Optional) Git: To pull the source code, you can also just download the ZIP.
- Rust: To compile the project. You can install rust from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Go the root directory of the project and execute `cargo run --release`.
This command will build and run the application in release mode, which is the quickest one.
This may take some time and generate quite the file footprint, a result from how rust works.

## Provided Executables

The easiest way is to run the provided executable. 
Only one architecture (x86) is provided and probably does not run on your system.


# Usage

## Login

Using pdec is relatively straight forward.
When you start the application you are greeted with three fields:

- Endpoint: This is the path of your CRCON.
Important is that you put only `https://my-crcon.com` and **NOT** `https://my-crcon.com/` or `https://my-crcon.com/api`.
The best way to get is to simply access your CRCON, copy the address **AND REMOVE THE TRAILING /**.
- Username: This is your username you use to log into CRCON.
- Password: This is your password you use to log into CRCON.

At this point a small note on security.
This project is horribly insecure and you should **never** just put in your username and password into a random application.
This project is no exception, you are trusting me not to just steal those credentials.
There are better ways to implement such a login system, some of which are supported by CRCON, but I can't be bothered to implement.
The best thing you can do is to look at the source code or ask someone who knows to look at it.
If you don't want to use your credentials ask your administrator to potentially set you up with a second, less powerful account.

## Display

Once logged in you can see the graph and two input fields. 
The sliders to the right are used to manipulate how the connection times are visualized, play around with them to figure out what they do.
The player id is the id of the player.
Below you can provide an alias so you know the name of the player.
Pdec does not by its own figure out the name of the player.

By restrictions of CRCON and my laziness whenever you add a player you download all the logs that are stored of them.
This can be quite a lot and can take some time to download.
Pdec might freeze for up to a minute but as long as the window doesn't close it most likely hasn't crashed.

After you have added a player you will probably have to zoom out quite a bit to the right as the x-axis starts at 1970.
Don't zoom out too far either or the application will crash.
