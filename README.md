# CareerBridge – AI-Powered Career Platform

> A comprehensive full-stack career development platform helping students and fresh graduates discover their perfect career path through intelligent recommendations, real job opportunities, and personalized learning resources.

## 🌟 Overview

CareerBridge is a production-ready web application combining a high-performance Rust backend with a modern React/Next.js frontend. The platform provides an end-to-end career development solution featuring:

- **Intelligent Job Matching** - AI-powered recommendations with real job details from database
- **Skill Gap Analysis** - Identify missing skills and get personalized learning paths
- **Progress Tracking** - Monitor applications and learning journey
- **Modern Authentication** - Email/password, Google OAuth, and GitHub OAuth
- **Beautiful UI/UX** - Dark theme with glassmorphism effects and smooth animations

### 🎯 Built For
- 🎓 **Students** seeking career guidance and internships
- 👨‍💼 **Fresh Graduates** transitioning into the workforce
- 📈 **Career Changers** exploring new opportunities
- 💼 **Job Seekers** looking for personalized recommendations

## 🎯 Key Features

### 🔐 Authentication & User Management
- **Multiple Login Options**
  - Traditional email/password authentication
  - Google OAuth integration
  - GitHub OAuth integration
  - Automatic account linking for OAuth users
- **Two-Step Onboarding**
  - Instant registration with JWT token
  - Complete profile later (education, skills, preferences)
  - Progress tracking with `profile_completed` flag
- **Secure Implementation**
  - JWT-based authentication (24-hour validity)
  - Argon2 password hashing
  - SQL injection prevention via parameterized queries
  - Protected API routes with middleware

### 💼 Job Discovery & Matching
- **Smart Job Recommendations**
  - AI-powered skill-based matching algorithm
  - Match scores (0-100%) showing compatibility
  - Identifies matched and missing skills
  - Filters by experience level (fresher, junior, mid)
  - Filters by job type (internship, part-time, full-time, freelance)
- **Real Job Details** (November 2025 Enhancement)
  - Comprehensive job descriptions from database
  - Actual responsibilities for each position
  - Real requirements and qualifications
  - Company benefits and perks
  - Salary ranges (min-max)
- **Interactive Search**
  - Real-time search across jobs, companies, and skills
  - Location filtering (all, remote, onsite)
  - Type filtering with multiple options
  - Detailed modal view for each job
  - Quick apply actions

### 📚 Learning & Development
- **Personalized Learning Resources**
  - Curated courses and tutorials
  - Skill gap-based recommendations
  - Relevance scoring algorithm
  - Free and paid resource filtering
  - Platform badges (Coursera, Udemy, etc.)
- **Progress Tracking**
  - Track completion percentage (0-100%)
  - Automatic completion detection
  - Learning history timeline
  - Resource bookmarking

### 📊 Career Analytics
- **Skill Gap Analysis**
  - Compare your skills vs role requirements
  - Calculate match percentage
  - Identify specific skill gaps
  - Get recommended courses to bridge gaps
  - Multi-job aggregated requirements
- **Application Tracking**
  - Track all job applications
  - Status updates (applied, interviewing, offered, rejected)
  - Timeline visualization
  - Notes and reminders
  - Application history

### 🎨 Modern UI/UX
- **Beautiful Design System**
  - Dark/light theme toggle with persistence
  - Glassmorphism effects with backdrop blur
  - Blue-purple gradient accents throughout
  - Smooth Framer Motion page transitions
  - Particle animations and visual effects
- **Responsive & Accessible**
  - Mobile, tablet, and desktop optimized
  - ARIA-compliant components
  - Keyboard navigation support
  - Loading states and skeletons
  - Toast notifications (Sonner)
- **Interactive Components**
  - 3D graphics with Three.js and React Three Fiber
  - Interactive globe visualizations
  - Animated carousels and marquees
  - Command palette (⌘K)
  - Drag & drop file uploads

### 🚀 Performance & Developer Experience
- **Optimized Stack**
  - Rust backend for high performance
  - Next.js 15 with Turbopack for fast development
  - React 19 with modern hooks
  - PostgreSQL for reliable data storage
  - Type-safe queries with SQLx
- **Developer Tools**
  - Comprehensive API testing suite (`api_tests.http`)
  - Full Rust documentation (`cargo doc`)
  - TypeScript for type safety
  - ESLint configuration
  - Hot module replacement

## 🛠️ Technology Stack

### Backend (Rust/Axum)
| Category | Technologies |
|----------|-------------|
| **Language** | Rust (Edition 2024) |
| **Web Framework** | Axum 0.8 - Modern async web framework |
| **Database** | PostgreSQL 14+ with SQLx for type-safe queries |
| **Authentication** | JWT (jsonwebtoken), OAuth2 (Google, GitHub) |
| **Security** | Argon2 password hashing, SQL injection prevention |
| **File Handling** | Axum Typed Multipart, pdf-extract for CV parsing |
| **Validation** | Validator crate with derive macros |
| **Async Runtime** | Tokio for high-performance async operations |
| **Logging** | Tracing with structured logging |

### Frontend (Next.js/React)
| Category | Technologies |
|----------|-------------|
| **Framework** | Next.js 15.3.5 with App Router & Turbopack |
| **UI Library** | React 19 with modern hooks |
| **Language** | TypeScript for type-safe development |
| **Styling** | Tailwind CSS 4 with custom utilities |
| **Components** | Radix UI (50+ accessible components), shadcn/ui architecture |
| **Animations** | Framer Motion 12, particle effects (@tsparticles) |
| **3D Graphics** | Three.js, React Three Fiber, drei helpers |
| **Forms** | React Hook Form with Zod validation |
| **Icons** | Lucide React, React Icons, Tabler Icons |
| **Data Viz** | Recharts for charts, dotted-map for maps |
| **UI Utilities** | Embla Carousel, React Dropzone, React Day Picker |
| **State** | Better Auth 1.3 for authentication state |
| **Database (Client)** | Drizzle ORM, LibSQL client |
| **Notifications** | Sonner for toast messages |
| **Theme** | next-themes for dark/light mode |

### Database & Infrastructure
- **PostgreSQL 14+** - Primary data store
- **Tables**: users, jobs, learning_resources, application_tracking, user_progress, notifications, skill_assessments
- **Enums**: experience_level, career_track, job_type, cost_indicator
- **Features**: UUID primary keys, JSONB support, full-text search capabilities

### Development Tools
- **API Testing**: REST Client (`api_tests.http`)
- **Documentation**: Rust docs (`cargo doc`), comprehensive READMEs
- **Code Quality**: ESLint, TypeScript compiler
- **Build Tools**: Cargo (Rust), npm/Turbopack (frontend)
- **Version Control**: Git with GitHub integration

## ⚡ Quick Start

### Prerequisites
- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **Node.js** 18+ and npm ([Install Node.js](https://nodejs.org/))
- **PostgreSQL** 14+ ([Install PostgreSQL](https://www.postgresql.org/download/))

### Backend Setup (Port 3000)

```bash
# Navigate to backend directory
cd backend

# Create and configure .env file
cat > .env << EOF
DATABASE_URL=postgresql://postgres:password@localhost:5432/career_bridge
JWT_SECRET=your-secret-key-change-in-production
FRONTEND_URL=http://localhost:3001
GOOGLE_CLIENT_ID=your-google-client-id
GOOGLE_CLIENT_SECRET=your-google-client-secret
GOOGLE_REDIRECT_URI=http://127.0.0.1:3000/api/auth/google/callback
GITHUB_CLIENT_ID=your-github-client-id
GITHUB_CLIENT_SECRET=your-github-client-secret
GITHUB_REDIRECT_URI=http://127.0.0.1:3000/api/auth/github/callback
EOF

# Create database
createdb -U postgres career_bridge

# Apply schema and seed data
psql -U postgres -d career_bridge -f schema.sql
psql -U postgres -d career_bridge -f seed_data.sql

# Build and run
cargo build --release
cargo run
```

Backend API: `http://localhost:3000`

### Frontend Setup (Port 3001)

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Run development server with Turbopack
npm run dev
```

Frontend App: `http://localhost:3001`

> **Important:** Both servers must be running simultaneously. The frontend (port 3001) communicates with the backend API (port 3000).

### Quick Test

```bash
# Test backend health
curl http://localhost:3000/

# Register a user
curl -X POST http://localhost:3000/api/register \
  -H "Content-Type: application/json" \
  -d '{"full_name":"Test User","email":"test@example.com","password":"test123"}'

# Visit frontend
open http://localhost:3001
```

## 📁 Project Architecture

```
career-bridge/
├── backend/                          # Rust/Axum API Server
│   ├── src/
│   │   ├── main.rs                  # Application entry point & server setup
│   │   ├── lib.rs                   # Library exports & crate documentation
│   │   ├── models.rs                # Database models, enums, & types
│   │   ├── auth.rs                  # JWT token generation & validation
│   │   ├── security.rs              # Argon2 password hashing
│   │   ├── errors.rs                # Centralized error handling
│   │   └── handlers/                # API endpoint handlers
│   │       ├── mod.rs               # Module exports
│   │       ├── types.rs             # Request/response DTOs
│   │       ├── auth.rs              # Registration & login endpoints
│   │       ├── oauth.rs             # Google & GitHub OAuth flows
│   │       ├── profile.rs           # Profile CRUD & CV upload
│   │       ├── jobs.rs              # Job recommendations & matching
│   │       ├── learning.rs          # Learning resource recommendations
│   │       ├── applications.rs      # Application tracking
│   │       └── progress.rs          # Progress tracking
│   ├── migrations/                   # Database migration scripts
│   ├── schema.sql                    # PostgreSQL schema definition
│   ├── seed_data.sql                 # Sample data (20 jobs with details)
│   ├── api_tests.http                # REST Client API test suite
│   ├── Cargo.toml                    # Rust dependencies & metadata
│   ├── README.md                     # Backend documentation
│   └── target/                       # Compiled binaries (gitignored)
│
├── frontend/                         # Next.js/React Application
│   ├── src/
│   │   ├── app/                     # Next.js App Router (pages)
│   │   │   ├── page.tsx             # Landing page (/)
│   │   │   ├── layout.tsx           # Root layout with providers
│   │   │   ├── template.tsx         # Page transition wrapper
│   │   │   ├── globals.css          # Global styles & animations
│   │   │   ├── global-error.tsx     # Error boundary
│   │   │   ├── auth/                # OAuth callback handling
│   │   │   ├── login/               # Login page with OAuth
│   │   │   ├── register/            # Registration page
│   │   │   ├── onboarding/          # Multi-step profile setup
│   │   │   ├── dashboard/           # User dashboard with recommendations
│   │   │   ├── jobs/                # Job marketplace & search
│   │   │   ├── resources/           # Learning resources catalog
│   │   │   ├── profile/             # Profile management & CV upload
│   │   │   └── demo/                # Demo/preview pages
│   │   │
│   │   ├── components/              # React components
│   │   │   ├── ui/                  # shadcn/ui components (50+)
│   │   │   │   ├── button.tsx
│   │   │   │   ├── card.tsx
│   │   │   │   ├── dialog.tsx
│   │   │   │   ├── form.tsx
│   │   │   │   ├── input.tsx
│   │   │   │   ├── select.tsx
│   │   │   │   ├── table.tsx
│   │   │   │   └── ...              # 40+ more components
│   │   │   │
│   │   │   ├── Navbar.tsx           # Navigation with theme toggle
│   │   │   ├── Footer.tsx           # Site footer with links
│   │   │   ├── JobCard.tsx          # Job listing card
│   │   │   ├── JobDetailsModal.tsx  # Job details popup
│   │   │   ├── ResourceCard.tsx     # Learning resource card
│   │   │   ├── OnboardingFlow.tsx   # Multi-step onboarding
│   │   │   ├── TestimonialScroll.tsx # Animated testimonials
│   │   │   ├── PageTransition.tsx   # Framer Motion transitions
│   │   │   ├── ThemeProvider.tsx    # Theme context
│   │   │   ├── ThemeToggle.tsx      # Dark/light mode switcher
│   │   │   ├── ErrorReporter.tsx    # Error tracking
│   │   │   └── ClientComponents.tsx # Client wrapper
│   │   │
│   │   ├── lib/                     # Utilities & helpers
│   │   │   ├── api.ts               # API client functions & types
│   │   │   ├── utils.ts             # General utility functions
│   │   │   ├── jobUtils.ts          # Job-related utilities
│   │   │   └── hooks/               # Shared custom hooks
│   │   │
│   │   ├── hooks/                   # Additional React hooks
│   │   │   └── use-mobile.ts        # Mobile detection hook
│   │   │
│   │   ├── assets/                  # Static assets (images, fonts)
│   │   ├── visual-edits/            # Visual editing tools
│   │   ├── App.jsx                  # Legacy app component
│   │   ├── App.css                  # Legacy styles
│   │   ├── main.jsx                 # Legacy entry point
│   │   └── index.css                # Legacy global styles
│   │
│   ├── public/                      # Static files (served directly)
│   ├── components.json              # shadcn/ui configuration
│   ├── next.config.ts               # Next.js configuration
│   ├── tsconfig.json                # TypeScript configuration
│   ├── tailwind.config.js           # Tailwind CSS configuration
│   ├── postcss.config.mjs           # PostCSS configuration
│   ├── eslint.config.js             # ESLint configuration
│   ├── vite.config.js               # Vite config (legacy)
│   ├── package.json                 # Dependencies & scripts
│   ├── README.md                    # Frontend documentation
│   └── node_modules/                # Dependencies (gitignored)
│
├── docs/                            # Additional documentation
│   ├── JOB_API_ENHANCEMENT_SUMMARY.md    # Job API updates overview
│   ├── FRONTEND_MIGRATION.md             # Frontend migration guide
│   ├── DEPLOYMENT_CHECKLIST.md           # Deployment instructions
│   └── BACKEND_CHANGES.md                # Backend changes log
│
├── .gitignore                       # Git ignore patterns
├── LICENSE                          # MIT License
└── README.md                        # This file (project overview)
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| `backend/src/handlers/` | All API endpoint logic organized by feature |
| `frontend/src/app/` | Next.js pages using App Router architecture |
| `frontend/src/components/ui/` | Reusable UI components from shadcn/ui |
| `backend/migrations/` | Database migration scripts for schema updates |
| `docs/` | Additional project documentation and guides |

## 🗄️ Database Schema

### Core Tables

#### **users**
Primary user account and profile data
```sql
id                UUID PRIMARY KEY
email             TEXT UNIQUE NOT NULL
password_hash     TEXT (nullable for OAuth users)
oauth_provider    VARCHAR(50) (google, github, or NULL)
oauth_id          VARCHAR(255) (provider's user ID)
avatar_url        TEXT (profile picture from OAuth)
full_name         TEXT NOT NULL
education_level   TEXT
experience_level  experience_level_enum
preferred_track   career_track_enum
profile_completed BOOLEAN DEFAULT false
skills            TEXT[] (array of skill names)
projects          TEXT[] (array of project names)
target_roles      TEXT[] (array of desired job titles)
raw_cv_text       TEXT (extracted from PDF or manual entry)
created_at        TIMESTAMPTZ DEFAULT NOW()
updated_at        TIMESTAMPTZ DEFAULT NOW()
```

#### **jobs**
Job listings with comprehensive details
```sql
id                SERIAL PRIMARY KEY
job_title         TEXT NOT NULL
company           TEXT NOT NULL
location          TEXT
job_description   TEXT
required_skills   TEXT[] (array of required skills)
experience_level  experience_level_enum
job_type          job_type_enum
salary_min        INTEGER
salary_max        INTEGER
responsibilities  TEXT[] (NEW: array of job responsibilities)
requirements      TEXT[] (NEW: array of qualifications)
benefits          TEXT[] (NEW: array of company benefits)
created_at        TIMESTAMPTZ DEFAULT NOW()
```

#### **learning_resources**
Curated courses and learning materials
```sql
id             SERIAL PRIMARY KEY
title          TEXT NOT NULL
platform       TEXT (e.g., "Coursera", "Udemy")
url            TEXT
related_skills TEXT[] (skills taught)
cost           cost_indicator_enum (free, paid)
created_at     TIMESTAMPTZ DEFAULT NOW()
```

#### **application_tracking**
Track user job applications
```sql
id         SERIAL PRIMARY KEY
user_id    UUID REFERENCES users(id)
job_id     INTEGER REFERENCES jobs(id)
status     TEXT (applied, interviewing, offered, rejected)
applied_at TIMESTAMPTZ DEFAULT NOW()
notes      TEXT
updated_at TIMESTAMPTZ DEFAULT NOW()
```

#### **user_progress**
Track learning resource completion
```sql
id                     SERIAL PRIMARY KEY
user_id                UUID REFERENCES users(id)
resource_id            INTEGER REFERENCES learning_resources(id)
completion_percentage  INTEGER (0-100)
started_at             TIMESTAMPTZ DEFAULT NOW()
completed_at           TIMESTAMPTZ (set when completion = 100)
```

#### **notifications**
User notifications (future feature)
```sql
id         SERIAL PRIMARY KEY
user_id    UUID REFERENCES users(id)
title      VARCHAR(255)
message    TEXT
type       VARCHAR(50)
is_read    BOOLEAN DEFAULT false
created_at TIMESTAMPTZ DEFAULT NOW()
```

#### **skill_assessments**
Skill proficiency tracking (future feature)
```sql
id                SERIAL PRIMARY KEY
user_id           UUID REFERENCES users(id)
skill_name        VARCHAR(255)
proficiency_level INTEGER (1-10 scale)
assessed_at       TIMESTAMPTZ DEFAULT NOW()
```

### Enums

```sql
-- User experience levels
experience_level_enum: fresher | junior | mid

-- Career paths
career_track_enum: web_development | data | design | marketing

-- Job types
job_type_enum: internship | part_time | full_time | freelance

-- Resource cost
cost_indicator_enum: free | paid
```

### Indexes

```sql
-- Performance optimization indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_oauth ON users(oauth_provider, oauth_id);
CREATE INDEX idx_jobs_skills ON jobs USING GIN(required_skills);
CREATE INDEX idx_applications_user ON application_tracking(user_id);
CREATE INDEX idx_progress_user ON user_progress(user_id);
```

### Sample Data

The `seed_data.sql` file includes:
- **20 diverse jobs** across multiple industries
- **Full job details** (responsibilities, requirements, benefits)
- **15+ learning resources** from various platforms
- Realistic salary ranges and skill requirements

## 🔌 API Reference

### Base URL
```
http://localhost:3000
```

### Authentication Flow

#### 1. Register (Traditional)
```http
POST /api/register
Content-Type: application/json

{
  "full_name": "John Doe",
  "email": "john@example.com",
  "password": "securepass123"
}

Response: { "token": "jwt_token", "user_id": "uuid", "message": "..." }
```

#### 2. OAuth (Google/GitHub)
```http
GET /api/auth/google    # Redirects to Google OAuth
GET /api/auth/github    # Redirects to GitHub OAuth

# After OAuth, redirects to:
http://localhost:3001/auth/callback?token=<JWT>&new_user=<true|false>
```

#### 3. Login
```http
POST /api/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "securepass123"
}

Response: { "token": "jwt_token", "user": {...} }
```

### Protected Endpoints

**Required Header:**
```
Authorization: Bearer <jwt_token>
```

#### Profile Management
```http
GET /api/profile                      # Get current user profile
POST /api/profile/complete            # Complete profile onboarding
PUT /api/profile                      # Update profile fields
POST /api/profile/cv/upload           # Upload CV PDF
```

#### Job Recommendations
```http
GET /api/jobs/recommendations?experience_level=junior&job_type=full_time&limit=10

Response: Array of jobs with:
- job (id, title, company, location, description, skills, salary)
- responsibilities[] (real from database)
- requirements[] (real from database)
- benefits[] (real from database)
- match_score (0-100)
- matched_skills[]
- missing_skills[]
```

#### Learning Resources
```http
GET /api/learning/recommendations     # Get personalized courses
```

#### Skill Gap Analysis
```http
GET /api/skill-gap/{target_role}      # Analyze skill gaps

Response:
- user_skills[]
- required_skills[]
- skill_gaps[]
- match_percentage
- recommended_resources[]
```

#### Application Tracking
```http
POST /api/applications                # Create application
GET /api/applications                 # Get all applications
PUT /api/applications/{id}            # Update application status
```

#### Progress Tracking
```http
POST /api/progress/resource/{id}/start           # Start learning
PUT /api/progress/resource/{id}                  # Update progress
GET /api/progress                                # Get all progress
```

### Query Parameters

| Endpoint | Parameter | Values |
|----------|-----------|--------|
| `/api/jobs/recommendations` | `experience_level` | `fresher`, `junior`, `mid` |
| | `job_type` | `internship`, `part_time`, `full_time`, `freelance` |
| | `limit` | Number (default: 10) |

### Response Codes

| Code | Meaning |
|------|---------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request (validation error) |
| 401 | Unauthorized (missing/invalid token) |
| 404 | Not Found |
| 409 | Conflict (e.g., email exists) |
| 500 | Internal Server Error |

### Testing

Use `backend/api_tests.http` with [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension in VS Code for interactive API testing.

```bash
# Or use curl
curl -X POST http://localhost:3000/api/register \
  -H "Content-Type: application/json" \
  -d '{"full_name":"Test User","email":"test@example.com","password":"test123"}'
```

## 🎨 Design System & UI Pages

### Design Principles

**Color Palette**
- **Background**: Dark theme (#0a0a0a) with subtle gradients
- **Primary**: Blue (#3b82f6) to Purple (#a855f7) gradients
- **Accents**: Cyan (#06b6d4), Pink (#ec4899), Emerald (#10b981)
- **Glassmorphism**: backdrop-blur with rgba overlays

**Typography**
- **Headings**: Poppins (bold, semi-bold)
- **Body**: Inter (regular, medium)
- **Monospace**: JetBrains Mono for code

**Effects & Animations**
- Glassmorphism with backdrop blur and borders
- Smooth Framer Motion page transitions
- Hover scale effects (1.05x)
- Floating animations for hero elements
- Infinite scroll marquees
- Particle backgrounds with @tsparticles
- 3D globe visualizations

**Component Architecture**
- Radix UI primitives for accessibility
- shadcn/ui component patterns
- Custom Tailwind utilities
- Responsive breakpoints (sm, md, lg, xl, 2xl)

### Application Pages

#### 🏠 **Landing Page** (`/`)
Landing experience with multiple sections:
- **Hero Section**: Animated headline, CTA buttons, 3D graphics
- **Features Showcase**: AI Career Roadmap, Job Matching, Resources, Progress
- **How It Works**: Step-by-step guide (Register → Explore → Apply → Track)
- **Testimonials**: Infinite scrolling success stories
- **Footer**: Company info, navigation links, social media

#### 📊 **Dashboard** (`/dashboard`)
Personalized control center:
- User summary card (name, education, track, experience)
- AI-powered job recommendations with match scores
- Curated learning resources
- Quick actions and statistics
- Recent activity timeline

#### 💼 **Jobs Marketplace** (`/jobs`)
Comprehensive job search:
- Real-time search bar (jobs, companies, skills)
- Advanced filters:
  - Location: All, Remote, Onsite
  - Type: Full-time, Part-time, Internship, Freelance
  - Experience: Fresher, Junior, Mid
- Job cards with:
  - Company and title
  - Salary range
  - Required skills badges
  - Match score indicator
  - Quick actions (View, Save, Apply)
- Job details modal with:
  - Full description
  - Responsibilities list
  - Requirements list
  - Benefits & perks
  - Apply button

#### 📚 **Learning Resources** (`/resources`)
Educational content hub:
- Skill-based filtering
- Cost filtering (Free, Paid, All)
- Resource cards with:
  - Course title and platform
  - Related skills
  - Direct link to course
  - Platform badge
- Responsive masonry grid layout

#### 👤 **Profile Management** (`/profile`)
Comprehensive profile editor:
- Personal information fields
- Skills management:
  - Add new skills with autocomplete
  - Remove existing skills
  - Visual skill badges
- Career preferences:
  - Education level
  - Experience level
  - Preferred career track
  - Target roles
- CV/Resume management:
  - PDF upload with drag & drop
  - Text extraction preview
  - Manual text entry option
  - File size validation

#### 🔐 **Authentication Pages**

**Login** (`/login`)
- Split-screen layout
- Email/password form with validation
- "Remember me" checkbox
- Google OAuth button
- GitHub OAuth button
- Link to registration page
- Gradient hero section

**Register** (`/register`)
- Streamlined sign-up (name, email, password only)
- Instant JWT token on registration
- Social auth options (Google, GitHub)
- Password strength indicator
- Link to login page

**Onboarding** (`/onboarding`)
Multi-step profile completion:
1. **Education**: Select education level
2. **Experience**: Choose experience level (fresher/junior/mid)
3. **Career Track**: Pick preferred track (web dev, data, design, marketing)
4. **Skills**: Add initial skills
5. **Projects**: List projects (optional)
6. **Target Roles**: Specify desired job titles

#### 🔄 **OAuth Callback** (`/auth/callback`)
Handles OAuth redirects with JWT token parsing and user routing

#### 🎭 **Demo Pages** (`/demo`)
Component showcases and feature previews

### Responsive Design

| Breakpoint | Width | Layout |
|------------|-------|--------|
| Mobile | < 640px | Single column, stacked |
| Tablet | 640-1024px | Two columns, condensed nav |
| Desktop | 1024-1536px | Three columns, full nav |
| Wide | > 1536px | Four columns, expanded |

### Accessibility Features

✅ ARIA labels on all interactive elements  
✅ Keyboard navigation support  
✅ Focus indicators for tab navigation  
✅ Screen reader compatible  
✅ Semantic HTML5 structure  
✅ High contrast ratios (WCAG AA compliant)  
✅ Alt text for all images  
✅ Form validation with error messages

## 🧮 Core Algorithms

### Job Matching Algorithm

Calculates compatibility between user and job postings:

```rust
match_score = (matched_skills_count / required_skills_count) × 100

Example:
User skills: ["JavaScript", "React", "Node.js"]
Job requires: ["JavaScript", "React", "CSS", "HTML"]
Matched: ["JavaScript", "React"] = 2
Required: 4
Score: (2/4) × 100 = 50%
```

**Features:**
- Case-insensitive skill matching
- Identifies matched and missing skills
- Returns sorted by match score (descending)
- Filters by experience level and job type

### Learning Resource Relevance

Prioritizes resources teaching new skills:

```rust
relevance_score = (new_skills_count / total_skills_taught) × 100

Example:
User skills: ["Python"]
Resource teaches: ["Python", "Django", "REST API"]
New skills: ["Django", "REST API"] = 2
Total taught: 3
Relevance: (2/3) × 100 = 66.7%
```

**Features:**
- Focuses on skill gap closure
- Considers user's current skill set
- Sorts by relevance score

### Skill Gap Analysis

Aggregates requirements across multiple jobs:

```rust
1. Find all jobs matching target role
2. Collect all required skills across jobs
3. Compare with user's current skills
4. Calculate: match_percentage = (matching_skills / required_skills) × 100
5. Recommend resources for missing skills

Example:
Target role: "Full Stack Developer"
Required skills (aggregated): ["JavaScript", "React", "Node.js", "PostgreSQL", "Docker"]
User has: ["JavaScript", "React"]
Gap: ["Node.js", "PostgreSQL", "Docker"]
Match: (2/5) × 100 = 40%
```

**Features:**
- Multi-job aggregation for comprehensive analysis
- Identifies specific skill gaps
- Provides targeted learning recommendations
- Shows match percentage for motivation

```

## 📊 Logging & Monitoring

### Backend Logging

Comprehensive tracing with structured logs:

```rust
// Log levels used:
INFO  - Business operations, successful actions
DEBUG - Detailed diagnostics, query parameters
WARN  - User errors, validation failures
ERROR - System errors, database failures
```

**What's Logged:**
- All authentication attempts (success/failure)
- Profile operations and updates
- Job recommendations with match scores
- Application tracking events
- OAuth flows and redirects
- Database query performance
- Error conditions with context

**Configuration:**
```bash
# Set log level
RUST_LOG=info cargo run      # Default (recommended)
RUST_LOG=debug cargo run     # Verbose debugging
RUST_LOG=warn cargo run      # Warnings and errors only
```

**Example Logs:**
```
INFO  User registered successfully: test@example.com
DEBUG Job recommendations - Experience: junior, Limit: 10
WARN  Login failed: Invalid password for user@example.com
ERROR Database connection pool exhausted
```

### Error Handling

**Smart Error Messages:**
- User-facing errors are friendly and actionable
- System errors are logged with full context
- Validation errors include field-specific messages
- OAuth errors redirect with clear feedback

**Example:**
```rust
// Database error (internal)
ERROR Failed to insert user: duplicate key value violates unique constraint

// User sees:
"An account with this email already exists. Please login or use a different email."
```
## 📚 Documentation

### Available Documentation

| Document | Description |
|----------|-------------|
| [`README.md`](README.md) | This file - comprehensive project overview |
| [`backend/README.md`](backend/README.md) | Detailed backend API documentation |
| [`frontend/README.md`](frontend/README.md) | Frontend architecture and setup guide |
| [`backend/api_tests.http`](backend/api_tests.http) | Interactive API testing examples |

### Code Documentation

**Backend (Rust):**
```bash
cd backend

# Generate and open HTML documentation
cargo doc --open --no-deps

# Includes:
# - Module documentation
# - Struct and enum definitions
# - Function signatures with examples
# - Error conditions
```

**Frontend (TypeScript):**
- Type definitions in `lib/api.ts`
- Component documentation in JSDoc comments
- README files in major directories

## 🤝 Contributing

We welcome contributions! Here's how to get started:

### Development Workflow

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes**
   - Follow existing code style
   - Add tests for new features
   - Update documentation
4. **Test your changes**
   ```bash
   # Backend
   cargo test
   cargo fmt -- --check
   cargo clippy
   
   # Frontend
   npm run lint
   npm run build
   ```
5. **Commit with clear messages**
   ```bash
   git commit -m "feat: add job filtering by salary range"
   ```
6. **Push and create Pull Request**
   ```bash
   git push origin feature/your-feature-name
   ```

## 📄 License

This project is licensed under the **MIT License**.

```
MIT License

Copyright (c) 2025 CareerBridge Team

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

<div align="center">

**Built with ❤️ by the From_Los_Santosh Team**

🚀 Empowering careers through technology

</div>

