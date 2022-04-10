# short-it
a simple url-shortener written in rust

## Features
+ config file instead of database
+ cli commands to change config variables
+ api routes to add/edit/delete or view shorts
+ get url metadata and show it in short link(not yet)
+ admin panel to create/edit/delete/analytics (not yet)
  + analytics(ip, referer, time) about short viewers

## Usage

```
short-it [OPTIONS]
Examples: 
    ./short-it --db_name short_db --db_user short_db_user --db_pass 'someP@ssw0rd'
    ./short-it --user admin --pass 'admin'
    
OPTIONS:
--db_name <Database_Name>
--db_pass <Database_Password>
--db_user <Database_Username>
--pass <Panel_Password>
--user <Panel_Username>
-h, --help                           Print help information
-V, --version                        Print version information
```
