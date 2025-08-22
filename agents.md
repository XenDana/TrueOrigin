# TrueOrigin Repository Summary

## Project Overview
TrueOrigin is a decentralized anti-counterfeit platform built on the Internet Computer Protocol (ICP). It enables brands to certify products using on-chain signatures, allows users to verify authenticity via QR codes, and provides rewards for verified interactions. The system serves three main stakeholders: Brand Owners, Resellers, and Users.

## Technology Stack
- **Frontend**: React, TypeScript, Vite, Tailwind CSS
- **Backend**: Rust, Internet Computer Protocol (ICP)
- **Key Libraries/Tools**: 
  - DFINITY Agent SDK for ICP integration
  - React Router for navigation
  - React Query for data fetching
  - Radix UI components for UI elements
  - Heroicons and Lucide React for icons

## Repository Structure
```
├── src/
│   ├── backend/
│   │   ├── src/
│   │   │   ├── lib.rs          # Main entry point
│   │   │   ├── api.rs          # API endpoints
│   │   │   ├── auth.rs         # Authentication logic
│   │   │   ├── error.rs        # Error handling
│   │   │   ├── global_state.rs # Data storage
│   │   │   ├── models.rs       # Data structures
│   │   │   ├── utils.rs        # Utility functions
│   │   │   └── icp.rs          # ICP-specific functionality
│   │   ├── Cargo.toml          # Backend dependencies
│   │   └── backend.did         # Candid interface definition
│   └── frontend/
│       ├── src/
│       │   ├── App.tsx         # Main application component
│       │   ├── components/     # Reusable UI components
│       │   ├── pages/          # Page components
│       │   ├── layouts/        # Layout components
│       │   ├── contexts/       # React context providers
│       │   ├── hooks/          # Custom React hooks
│       │   ├── providers/      # Application providers
│       │   └── api/            # API integration layer
│       └── package.json        # Frontend dependencies
├── README.md                   # Project overview and setup instructions
└── project.md                  # Feature requirements and roadmap
```

## Core Features

### Backend (Rust/ICP Canister)
1. **Organization Management**
   - Creation, updates, and lookup of organizations
   - ECDSA key pair generation for organizations

2. **Product Registration**
   - Product registration system with metadata storage
   - Unique product serial number generation

3. **User Management**
   - User registration and management
   - Authentication & authorization (organization-based)

4. **Verification System**
   - Product authenticity verification via QR codes
   - Fraud detection through signature timestamp and usage history
   - Unique code generation for QR codes

5. **Security**
   - ECDSA signatures for cryptographic verification
   - Organization-based access control
   - Secure random number generation using ICP's `raw_rand()`

6. **Data Storage**
   - In-memory storage using thread-safe collections
   - Persistent storage using `ic-stable-structures`
   - StableBTreeMaps for efficient key-value storage

### Frontend (React/TypeScript)
1. **Public Verification Interface**
   - QR code scanning functionality
   - Instant feedback on product authenticity
   - Product information display
   - Reward information display

2. **Brand Dashboard**
   - Product batch upload (CSV/manual)
   - Verification statistics
   - Certificate management
   - Feedback/sentiment analysis reports

3. **Role-Based Access Control**
   - Brand Owner layout and pages
   - Reseller layout and pages
   - Protected routes based on user roles

4. **UI Components**
   - Reusable component library (Card, Table, Modal, etc.)
   - Responsive design with Tailwind CSS
   - Dark mode support

## Key Integrations
1. **ICP Blockchain**
   - ECDSA for Product & Reseller Certification
   - HTTPS outcalls for external API integrations
   - Canister-based architecture for backend logic

2. **Reward System**
   - ICP/ETH token rewards for successful verifications
   - Rate limiting to prevent abuse
   - Referral tracking through QR codes

3. **AI Integration**
   - Sentiment analysis via HTTPS outcalls to AI APIs
   - Feedback collection after verification

## Development Setup
1. **Prerequisites**
   - Rust (latest stable version)
   - DFX (ICP development framework)
   - Node.js (version 18 or higher)
   - pnpm (package manager)

2. **Running Locally**
   ```bash
   # Start the ICP replica
   dfx start --background
   
   # Install frontend dependencies
   pnpm install
   
   # Add WASM target for Rust
   rustup target add wasm32-unknown-unknown
   
   # Deploy canisters
   dfx deploy
   ```

3. **Development Commands**
   - `pnpm dev` - Start frontend development server
   - `dfx generate TrustOrigin_backend` - Generate new candid interface
   - `dfx deploy` - Deploy canisters to local replica

## Architecture Highlights
1. **Backend Architecture**
   - Service-oriented design with domain-specific modules
   - Repository pattern for data access
   - Standardized error handling
   - Comprehensive logging with emoji tagging

2. **Frontend Architecture**
   - Component-based UI with React
   - Lazy loading for performance optimization
   - Context providers for global state management
   - Protected routes for role-based access control
   - React Query for server state management

3. **Security Features**
   - Organization-based access control
   - ECDSA signatures for tamper-proof certificates
   - Secure QR code generation with unique verification codes
   - Rate limiting for reward distribution

## Future Enhancements
1. Advanced analytics and reporting
2. Enhanced reward mechanisms
3. Additional blockchain integrations
4. Improved sentiment analysis capabilities
5. Indexing for more efficient queries
6. Pagination support for large datasets

This repository provides a complete foundation for a decentralized product authentication system with both backend and frontend components, ready for extension and customization.
