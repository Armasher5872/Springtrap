# smashline dev plugin template

A Smashline Development Plugin template

To use, download and install the development version of smashline_hook. BE SURE TO DISABLE both the normal smashline_hook AND nro_hook plugins.

Then, make sure you set your Switch's IP using cargo skyline set-ip your.ip.here and run pushdev.bat to automatically build and push the development plugin to your Switch.
 
Some things to keep in mind:

In your lib.rs, you must change the name in #[skyline::main(name = "Replace with the same name as your plugin")] to be the same as your main plugin's.
For example: in The WuBor Patch's lib.rs, the header reads #[skyline::main(name = "the_wubor_patch")].
The header must be the same in your development plugin.

Any and all ACMD changes must go into custom/mod.rs due to the limitation of only allowing one #{installer] function in the development plugin.

To reload the dev plugin, press L + R + DPad Up. DON'T DO THIS DURING A MATCH, IT WILL MOST LIKELY CRASH.
Return to the Character Select Screen before performing this command

For any and all other questions, please refer to the smashline wiki, specifically https://github.com/blu-dev/smashline/wiki/The-Development-Plugin.
For any further questions, feel free to DM WuBoy#0238 on Discord.

Special thanks to blujay for their work on Smashline and Ultimate modding in general.