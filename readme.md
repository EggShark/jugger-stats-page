# Jugger-Stats-Page
(better name pending)

The goal is for this to be a statics and stats page for all jugger leauges. Current plan is to only support quick and general team stats based on points, and time to get points.
Once real time statsa are actually tracked (i.e who downs who in games and pomf ussage data) I plan to add those values into this website
## Set up
This is the setup for the back end to work and to start the server. Creation of the tables is nesscary for the project to compile but not run due to the use of `sqlx::query!` macro.
### Postgres database
Have a valid postgres server running connect to it and then run `db/tables.sql` to initlize the proper tables
### ENV setup
create an .env file that looks like:
```bash
DATABASE_URL="postgresql://username@server/database"
PASTEO_SECRET="REALYSECURESECRETTHATISCOOLYEPSO"
```
`PASTEO_SECRET` must be **EXACTLY** 32 characters long and is used to create encrpyted user session tokens
### Type Script
It should be as simple as running `tsc` and having the files be compiled