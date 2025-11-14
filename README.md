# CareerBridge – AI-Powered Career Platform

CareerBridge is a modern, feature-rich web application designed to help students and fresh graduates discover their perfect career path through AI-powered recommendations, curated job opportunities, and personalized learning resources.  
The interface features a sleek dark theme with glassmorphism effects, blue-purple gradient accents, and smooth animations to deliver a professional yet approachable experience.

## Highlights

- **Dynamic landing page** with hero section, feature showcase, how-it-works guide, and scrolling testimonials
- **Interactive dashboard** displaying personalized job recommendations, learning resources, and profile match indicators
- **Advanced jobs marketplace** with real-time search, location/type filters, and detailed job modal with full descriptions
- **Learning resources hub** featuring skill and cost-based filters, curated course cards, and direct platform links
- **Comprehensive profile management** for editing personal info, skills, career preferences, and CV content
- **Modern authentication** with split-screen login/register flows, Google OAuth integration, and gradient hero sections
- **Professional footer** inspired by industry leaders, with contact info, navigation links, and social media integration
- **Smooth animations** and polished hover states creating cohesive user experience across all pages

## Tech Stack

### Frontend
- **Next.js 15.3.5** with Turbopack for blazing-fast development
- **React 19** with client-side interactivity
- **Tailwind CSS 4** with custom glassmorphism and gradient utilities
- **Framer Motion** for smooth page transitions and animations
- **Radix UI** components for accessible, unstyled primitives
- **Lucide React** for beautiful, consistent iconography
- **TypeScript** for type-safe development
- Custom CSS animations for floating effects and infinite scrolling

### Backend
- **Rust (Axum)** for high-performance API server
- **PostgreSQL** for reliable data storage
- **SQLx** for type-safe database queries
- **JWT** for secure authentication
- **OAuth2** for Google and GitHub authentication
- **bcrypt** for password hashing

## Quick Start

### Backend Setup

```bash
# Clone the repository
git clone https://github.com/tamim2763/career-bridge.git

# Navigate to backend directory
cd career-bridge/backend

# Setup PostgreSQL database
createdb career_bridge

# Run database schema
psql -U your_user -d career_bridge < schema.sql

# Load seed data (includes 20 jobs with full details)
psql -U your_user -d career_bridge < seed_data.sql

# Configure environment variables
# Create .env file with:
# DATABASE_URL=postgres://user:password@localhost/career_bridge
# JWT_SECRET=your-secret-key
# GOOGLE_CLIENT_ID=your-google-client-id
# GOOGLE_CLIENT_SECRET=your-google-client-secret
# GITHUB_CLIENT_ID=your-github-client-id
# GITHUB_CLIENT_SECRET=your-github-client-secret

# Build and run the backend
cargo run
```

Backend API will be available at `http://localhost:3000`

### API Testing

Use the included `backend/api_tests.http` file with REST Client extension in VS Code, or use curl:

```bash
# Health check
curl http://localhost:3000/

# Register a user
curl -X POST http://localhost:3000/api/register \
  -H "Content-Type: application/json" \
  -d '{"full_name":"John Doe","email":"john@example.com","password":"pass123"}'

# Login and get token
curl -X POST http://localhost:3000/api/login \
  -H "Content-Type: application/json" \
  -d '{"email":"john@example.com","password":"pass123"}'

# Get job recommendations (requires token)
curl http://localhost:3000/api/jobs/recommendations \
  -H "Authorization: Bearer YOUR_TOKEN"
```

See `backend/api_tests.http` for comprehensive API examples.

### Frontend Setup

```bash
# Navigate to frontend directory
cd career-bridge/frontend

# Install dependencies
npm install

# Run development server
npm run dev
```

Navigate to `http://localhost:3001` to explore the application.

> **Note:** The frontend runs on port 3001, while the backend API runs on port 3000. Make sure both servers are running for the application to work properly.

## Project Structure

```
career-bridge/
├── backend/
│   ├── src/
│   │   ├── main.rs                 # Application entry point
│   │   ├── lib.rs                  # Library exports
│   │   ├── models.rs               # Database models & enums
│   │   ├── auth.rs                 # JWT authentication
│   │   ├── security.rs             # Password hashing & validation
│   │   ├── errors.rs               # Error handling
│   │   └── handlers/
│   │       ├── mod.rs              # Handler module exports
│   │       ├── auth.rs             # Auth endpoints (login/register)
│   │       ├── oauth.rs            # OAuth flows (Google/GitHub)
│   │       ├── profile.rs          # Profile management
│   │       ├── jobs.rs             # Job recommendations
│   │       ├── learning.rs         # Learning resources
│   │       ├── applications.rs     # Application tracking
│   │       ├── progress.rs         # Progress tracking
│   │       └── types.rs            # Request/response types
│   ├── migrations/
│   │   └── 001_add_job_details.sql # Database migrations
│   ├── schema.sql                  # Database schema
│   ├── seed_data.sql               # Sample data (20 jobs)
│   ├── api_tests.http              # API test examples
│   ├── Cargo.toml                  # Rust dependencies
│   └── README.md                   # Backend documentation
├── frontend/
│   ├── src/
│   │   ├── app/
│   │   │   ├── dashboard/          # User dashboard with recommendations
│   │   │   ├── jobs/               # Job listings with search & filters
│   │   │   ├── resources/          # Learning resources catalog
│   │   │   ├── profile/            # User profile management
│   │   │   ├── login/              # Authentication - sign in
│   │   │   ├── register/           # Authentication - sign up
│   │   │   ├── onboarding/         # Multi-step onboarding flow
│   │   │   ├── page.tsx            # Landing page
│   │   │   ├── layout.tsx          # Root layout wrapper
│   │   │   ├── template.tsx        # Page transition wrapper
│   │   │   └── globals.css         # Global styles & animations
│   │   ├── components/
│   │   │   ├── ui/                 # Reusable UI components (Radix UI)
│   │   │   ├── Navbar.tsx          # Navigation component
│   │   │   ├── Footer.tsx          # Footer with links & contact
│   │   │   ├── TestimonialScroll.tsx  # Infinite scrolling testimonials
│   │   │   ├── OnboardingFlow.tsx  # Multi-step onboarding component
│   │   │   ├── PageTransition.tsx  # Framer Motion page transitions
│   │   │   ├── ThemeToggle.tsx     # Dark/light theme switcher
│   │   │   ├── ThemeProvider.tsx   # Theme context provider
│   │   │   ├── JobCard.tsx         # Job listing card
│   │   │   ├── ResourceCard.tsx    # Learning resource card
│   │   │   └── JobDetailsModal.tsx # Job details modal
│   │   ├── lib/
│   │   │   ├── api.ts              # API client & types
│   │   │   └── utils.ts            # Utility functions
│   │   └── hooks/                  # Custom React hooks
│   ├── public/                      # Static assets
│   ├── package.json
│   ├── next.config.ts
│   └── README.md
├── JOB_API_ENHANCEMENT_SUMMARY.md  # API enhancement overview
├── FRONTEND_MIGRATION.md           # Frontend migration guide
├── DEPLOYMENT_CHECKLIST.md         # Deployment instructions
├── README.md                       # This file
└── .gitignore
```

## Current Features

### **Backend API**
- RESTful API with JWT authentication
- OAuth2 integration (Google, GitHub)
- **Enhanced Job API** with responsibilities, requirements, and benefits
- Personalized job recommendations with match scoring
- Skill gap analysis for target roles
- Learning resource recommendations
- Application tracking system
- Progress tracking for learning resources
- Secure password hashing and token management

### **Landing Page**
- Engaging hero section with AI-powered platform messaging
- Feature cards showcasing AI Career Roadmap, Job Matching, Learning Resources, and Progress Tracking
- Step-by-step "How CareerBridge Works" guide
- Infinite scrolling testimonials from successful users
- Comprehensive footer with company info, navigation, and social links

### **Dashboard**
- Personalized user summary with education, track, and experience
- AI-powered job recommendations matched to user profile
- Curated learning resources tailored to career goals
- Interactive job details modal with full descriptions

### **Jobs Marketplace**
- Real-time search across jobs, companies, and skills
- Advanced filters (location: all/remote/onsite, type: full-time/part-time/contract/internship)
- Responsive job cards with salary, skills, and quick actions
- Detailed job modal with **real** responsibilities, requirements, and benefits from database
- AI-powered job matching with skill gap analysis
- Match score indicators showing compatibility with user profile

### **Learning Resources**
- Skill-based and cost-based filtering (free/paid)
- Clean card layouts with platform badges
- Direct links to external learning platforms
- Responsive grid layout for optimal viewing

### **Profile Management**
- Editable personal information fields
- Dynamic skills management with add/remove functionality
- Career preferences (education, experience, track)
- CV/Resume file upload (PDF, DOC, DOCX) with file management

### **Authentication**
- Split-screen design with gradient hero sections
- Login page with email/password and "Remember me" option
- Register page with streamlined sign-up (name, email, password only)
- Multi-step onboarding flow after registration (education, experience, career track)
- Google OAuth integration with official branding
- Elegant "OR" divider for social auth options
- Mobile-responsive forms with validation

### **UI/UX Excellence**
- Consistent dark/light theme toggle with glassmorphism effects
- Blue-purple gradient accents throughout
- Smooth Framer Motion page transitions between routes
- Subtle hover and tap animations on buttons and cards
- Responsive design for all screen sizes
- Accessible components with proper ARIA labels

## Design System

- **Colors**: Dark background with blue (#3b82f6) to purple (#a855f7) gradients
- **Effects**: Glass morphism with backdrop blur, subtle borders, and glow effects
- **Typography**: Poppins for headings, Inter for body text
- **Animations**: Floating elements, infinite scroll, hover scale effects
- **Components**: Radix UI primitives with custom Tailwind styling

## Pages Overview

1. **Home (`/`)** – Landing page with hero, features, how-it-works, testimonials, and footer
2. **Dashboard (`/dashboard`)** – Personalized overview with job and resource recommendations
3. **Jobs (`/jobs`)** – Searchable job listings with advanced filters
4. **Resources (`/resources`)** – Curated learning materials with skill filters
5. **Profile (`/profile`)** – User profile editor with skills management and CV upload
6. **Login (`/login`)** – Sign in with email/password or Google OAuth
7. **Register (`/register`)** – Create account with name, email, and password
8. **Onboarding (`/onboarding`)** – Multi-step onboarding flow for education, experience, and career track

Each page features smooth client-side navigation and maintains state during demo interactions.

## API Endpoints

### Authentication
- `POST /api/register` - Create new user account
- `POST /api/login` - Login with email/password
- `GET /api/auth/google` - Initiate Google OAuth flow
- `GET /api/auth/github` - Initiate GitHub OAuth flow

### Profile Management (Protected)
- `GET /api/profile` - Get user profile
- `POST /api/profile/complete` - Complete profile onboarding
- `PUT /api/profile` - Update profile information

### Jobs (Protected)
- `GET /api/jobs/recommendations` - Get personalized job recommendations
  - Query params: `experience_level`, `job_type`, `limit`
  - Returns: Jobs with match scores, responsibilities, requirements, benefits

### Learning Resources (Protected)
- `GET /api/learning/recommendations` - Get personalized learning resources

### Skill Gap Analysis (Protected)
- `GET /api/skill-gap/{target_role}` - Analyze skill gaps for target role
  - Returns: Required jobs, skills gap, recommended resources

### Application Tracking (Protected)
- `POST /api/applications` - Create job application
- `GET /api/applications` - Get user's applications
- `PUT /api/applications/{id}` - Update application status

### Progress Tracking (Protected)
- `POST /api/progress/resource/{id}/start` - Start learning resource
- `PUT /api/progress/resource/{id}` - Update progress percentage
- `GET /api/progress` - Get all user progress

All protected endpoints require `Authorization: Bearer <token>` header.

## Recent Enhancements

### Job API Enhancement (November 2025)
The Job API has been significantly improved to provide real, database-driven job details:
- **New Fields**: Each job now includes `responsibilities`, `requirements`, and `benefits` arrays
- **Real Data**: All 20 sample jobs populated with realistic, role-specific information
- **Better UX**: Users see actual job details instead of generic templates
- **Data Integrity**: Single source of truth in database, no client-side generation

See `JOB_API_ENHANCEMENT_SUMMARY.md` and `FRONTEND_MIGRATION.md` for complete details.

## Future Enhancements

- AI-powered career roadmap generation with personalized milestones
- Advanced job search with salary negotiation insights
- Real-time notifications for new job matches
- Social features (networking, mentorship matching)
- Resume builder and interview preparation tools
- Company reviews and culture insights
- Skill assessments and certifications tracking

## Documentation

For more detailed information, see:
- **`backend/BACKEND_CHANGES.md`** - Complete backend API documentation and changes
- **`JOB_API_ENHANCEMENT_SUMMARY.md`** - Overview of job API enhancements
- **`FRONTEND_MIGRATION.md`** - Frontend integration guide for new API features
- **`DEPLOYMENT_CHECKLIST.md`** - Step-by-step deployment instructions
- **`backend/api_tests.http`** - Comprehensive API testing examples

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.

---

Built with ❤️ by the CareerBridge Team

