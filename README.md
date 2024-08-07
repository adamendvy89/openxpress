# OpenXpress

OpenXpress is a versatile platform offering Docker container hosting with features designed to simplify and enhance web application deployment. Whether you're showcasing projects, launching startups, or integrating CI/CD pipelines, OpenXpress has you covered.

## Features

- 🐳 **Docker Container Hosting**: Deploy and manage applications using Docker containers.
- 🔄 **Webhook Logger**: Log and monitor webhook events in real-time.
- 🔗 **Free Subdomain URL**: Get a unique subdomain on `openxpress.cloud` for each application.
- 🌐 **Expose Local Apps Online**: Make local applications accessible online through secure Docker hosting.
- 🚪 **Multiple Port Support**: Configure multiple ports for handling various services.
- 🔒 **Auto SSL**: Free SSL certificates for secure connections.

## How It Works

1. 🚀 **Get OpenXpress Auth Token**: Sign up and obtain your authentication token.
2. 📦 **Create Docker Container**: Upload a ZIP file containing your Dockerfile for hosting.
3. 🚪 **Define Ports**: Specify ports for application access.
4. 🌐 **Receive Subdomain**: Each port gets a unique subdomain (`subdomain.openxpress.cloud`).

## Use Cases

- 🖼️ **Showcasing Projects to Clients**: Deploy projects for client previews.
- 🚀 **Launching Startups**: Cost-effective hosting for startups.
- 🛠️ **Development and Testing**: Create isolated environments for development and testing.
- 📚 **Educational Projects**: Share projects and tutorials online.
- 🔄 **CI/CD Integration**: Automate deployment processes.
- 🔗 **Webhook API Testing**: Test API integrations.

Explore the full potential of your applications with OpenXpress. [Get Started Now!](https://openxpress.cloud)

## How Build OpenXpress from Source

OpenXpress is a powerful and flexible application for managing and deploying your projects with ease. 

Requirement
```
cargo version:  1.79.0 (ffa9cf99a 2024-06-03)
```

## How to Deploy and Run OpenXpress Client in Rust Language Code

To get started with running the OpenXpress Rust project, follow these steps:

1. **Install Rust**: If you don't already have Rust installed, download and install it from [the official Rust website](https://www.rust-lang.org/).

2. **Clone the OpenXpress repository**: Clone the project from the GitHub repository.
    ```bash
    git clone https://github.com/adamendvy89/openxpress.git
    cd openxpress
    ```

3. **Build and run the project**: Use Cargo to build and run the project.
    ```bash
    cargo build
    cargo run
    ```
## How to Use the OpenXpress Client App

Follow these instructions to download, extract, and run the OpenXpress builder app:
1. **Register to OpenXpress Website**:
   - Get `Username` and `Password` for login from [Register Page](https://openxpress.cloud/register).
   - This credential will used for openxpress client app.
     
2. **Download and extract the OpenXpress package**:
   - Ensure you have downloaded the latest OpenXpress package from [Download Page](https://openxpress.cloud/download).
   - Extract the package to a directory of your choice.

3. **Open a command prompt and navigate to the extracted directory**:
   ```bash
   cd path\to\extracted\directory

4. **To start OpenXpress on a single port**:
  ```
  openxpress.exe -u <Your Username> -p <Your Password> -P 80 -f path\to\extracted\director\openxpress\examples\simple-php-master.zip
  ```
5. **To start OpenXpress on multiple ports**:
  ```
  openxpress.exe -u <Your Username> -p <Your Password> -P 80,81 -f path\to\extracted\director\openxpress\examples\simple-php-master.zip
  ```
6. Press `Enter` and wait until the terminal returns the subdomain. This process will depend on the zip file size you upload.
  ```
  Your URL is ready: 
  subdomain for port 80: <uniqid>.openxpress.cloud
  subdomain for port 81: <uniqid>.openxpress.cloud
  ```

Replace `<Your Username>` and `<Your Password>` with your actual username and password. The -P flag specifies the port (80 in this case), and the -f flag specifies the path to the file you want to deploy.

**Note:** The zip file (simple-php-master.zip) must have a `Dockerfile` in its root directory.

Example file structure for **simple-php-master.zip**
```
simple-php-master/
├── Dockerfile
└── src/
    ├── index.php
    ├── webhook.php
    ├── logs/
    │   └── webhook.log
    ├── config/
    │   └── config.php
    └── public/
        ├── index.html
        └── style.css
```

Example code for **Dockerfile**
```
# Use the official PHP image with Apache
FROM php:8.0-apache

# Copy custom php.ini file to the container
COPY php.ini /usr/local/etc/php/php.ini

# Copy the PHP script into the container
COPY src/ /var/www/html/

# Set permissions
RUN chown -R www-data:www-data /var/www/html

# Enable Apache modules if needed
RUN a2enmod rewrite

# Restart Apache to apply changes
RUN service apache2 restart

```

## How to Use Webhook Logger

Example code for webhook logger is on [**simple-php-master.zip**](https://github.com/adamendvy89/openxpress/blob/master/examples/simple-php-master.zip)

Send HTTP Request to URL:
```
https://<Your uniqid>.openxpress.cloud/webhook.php
```
See webhook Log with URL:
```
https://<Your uniqid>.openxpress.cloud/logs/webhook.log
```

## API Documentation

For more examples and features, you can visit the [OpenXpress API Docs](https://api.openxpress.cloud/api/docs/).


