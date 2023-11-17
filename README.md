# Transparent Ebook Copyright Management System (TECMaS)

A transparent ebook copyright management system aims to better protect the intellectual property rights of authors while also providing readers with a smooth and intuitive user experience. This system could allow publishers and writers to securely distribute their ebooks while preventing unauthorized copying.

The system would likely involve adding digital watermarking technology directly into the ebook files. This special encoding can embed copyright notices, authorship identification, and other metadata within the text and images of the ebook. The watermarks are invisible to the reader but allow the file to be tracked and usage monitored.

For authorized ebook formats, the watermarking could be paired with digital rights management (DRM) restrictions that limit copying, printing, and sharing depending on the publisher's policies. However readers access the ebooks - whether downloading to a reading app or browsing via web browser - the files stay secure.

Importantly, the system remains seamless and easy-to-use on the reader side. There is no interruption to the book reading experience even as the usage is tracked behind the scenes via the watermarking data. Readers don't have to login or authenticate constantly.

The goal is to balance strong copyright controls for publishers with an intuitive, flowing reading interface. With a transparent system, both authors and readers can benefit from technological protection of intellectual property.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.
