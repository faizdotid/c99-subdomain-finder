# C99 Subdomain Finder

`c99-subdomain-finder.exe` is a command-line tool used to find subdomains for a list of domains using the C99 API.

## Features
- Scan multiple domains for subdomains using the C99 Subdomain Finder API.
- Supports concurrent requests for faster processing.
- Option to specify the output file to save the subdomain results.

## Usage

c99-subdomain-finder.exe [OPTIONS] --apikey `<APIKEY>` --output `<OUTPUT>` --input `<INPUT>`


### Options

| Option                          | Description                                  |
|----------------------------------|----------------------------------------------|
| `-a, --apikey <APIKEY>`          | API key for C99 Subdomain Finder (required). |
| `-o, --output <OUTPUT>`          | Output file to save subdomains (required).   |
| `-c, --concurrency <CONCURRENCY>`| Number of concurrent requests (default: 10). |
| `-i, --input <INPUT>`            | Input file containing a list of domains (required). |
| `-h, --help`                     | Print help message.                          |
| `-V, --version`                  | Print version information.                   |

### Example

c99-subdomain-finder.exe --apikey YOUR_API_KEY --output results.txt --input domains.txt

In this example:
- `YOUR_API_KEY` is your C99 Subdomain Finder API key.
- `results.txt` is the file where the discovered subdomains will be saved.
- `domains.txt` contains the list of domains to scan.

### Requirements
- C99 API key
- A valid file containing domains to scan.
