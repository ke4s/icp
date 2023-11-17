# Transparent Ebook Copyright Management System (TECMaS)

# Project Description
A transparent ebook copyright management system aims to better protect the intellectual property rights of authors. This system could allow publishers and writers to securely distribute their ebooks while preventing unauthorized copying.

The system would likely involve adding digital watermarking technology directly into the ebook files. This special encoding can embed copyright notices, authorship identification, and other metadata within the text and images of the ebook.

For authorized ebook formats, the watermarking could be paired with digital rights management restrictions that limit copying, printing, and sharing depending on the publisher's policies. 

The goal is to balance strong copyright controls for publishers. With a transparent system, authors can benefit from technological protection of intellectual property.

# Tracks
## Goals
- Refinement of Blockchain Integration:

Address challenges related to integrating blockchain technology for decentralized rights management.
Implement smart contract-based permissions and access controls to enhance security.
- Enhancement of Rust Proficiency:

Further develop proficiency in Rust programming language, particularly focusing on the ownership system.
- User Interface (UI) and User Experience (UX) Enhancement:

Improve the frontend development by enhancing the user interface and overall user experience.
Ensure that the system is intuitive and easy to use for both authors and publishers.
- Testing and Bug Fixes:

Conduct thorough testing of the entire system to identify and fix any bugs or issues.
Ensure that the system operates smoothly under various conditions.
## Milestones
- Milestone 1: Blockchain Integration

Task 1: Address technical complexities related to blockchain integration.

Task 2: Implement smart contract-based permissions and access controls.
- Milestone 2: Rust Proficiency

Task 1: Conduct Rust workshops or training sessions to enhance team members' proficiency.

Task 2: Apply advanced Rust concepts to optimize and secure the codebase.
- Milestone 3: UI/UX Enhancement

Task 1: Collect user feedback and identify areas for improvement in the current UI/UX.

Task 2: Implement design changes to enhance the overall user experience.
- Milestone 4: Testing and Bug Fixes

Task 1: Develop and execute comprehensive test cases to identify bugs and vulnerabilities.

Task 2: Address and fix any issues identified during testing.

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

# Team Participants
## GitHub names - Discord IDs
- ahmetfalan - ahmetfalan #8272
- rasnesakam - ne10
- burcakseven - burcaks
- ke4s - ke4s

# Cannister ID
bd3sg-teaaa-aaaaa-qaaba-cai

# Feedback Part

## What We Have Learned
- We expanded our understanding substantially despite some technological limitations. The research and discovery process itself yielded invaluable insights that can guide our ongoing efforts to develop transparent, robust, and easy-to-use ebook copyright management systems.

## What Were The Challenges We Run Into
- Integrating blockchain technology for decentralized rights management proved more difficult than anticipated. We underestimated the technical complexities of implementing smart contract-based permissions and access controls.

- Rust offered memory safety benefits but has a steep learning curve coming from other languages. Mastering Rust's ownership system took time.

## What Are We Proud Of
- Despite limited time and experience, our team is proud to have rapidly learned emerging skills like hashing and Rust to build an end-to-end ebook security prototype. This successful spike positions us well for developing a robust transparent copyright system in the future.
