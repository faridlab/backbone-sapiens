# Business Requirements Document (BRD)
## Sapiens - User Management System (UMS)

**Document Version**: 2.0
**Last Updated**: November 19, 2025
**Author**: Farid Hidayat (CEO, StartApp)
**Status**: Final Draft
**Approval Date**: TBD

---

## 📋 Table of Contents

1. [Document Information](#1-document-information)
2. [Project Overview](#2-project-overview)
3. [Business Context](#3-business-context)
4. [Stakeholders](#4-stakeholders)
5. [Functional Requirements](#5-functional-requirements)
6. [Non-Functional Requirements](#6-non-functional-requirements)
7. [User Stories & Use Cases](#7-user-stories--use-cases)
8. [Data Requirements](#8-data-requirements)
9. [Database Schema Specifications](#9-database-schema-specifications)
10. [Integration Requirements](#10-integration-requirements)
11. [API Specifications](#11-api-specifications)
12. [Technical Constraints](#12-technical-constraints)
13. [Security Requirements](#13-security-requirements)
14. [Success Criteria](#14-success-criteria)
15. [Assumptions & Dependencies](#15-assumptions--dependencies)
16. [Risks & Mitigation](#16-risks--mitigation)
17. [Timeline & Milestones](#17-timeline--milestones)
18. [Budget & Resources](#18-budget--resources)
19. [Testing & Validation Framework](#19-testing--validation-framework)
20. [Glossary & References](#20-glossary--references)

---

## 1. Document Information

### 1.1 Document Control

| Attribute | Value |
|-----------|-------|
| **Document Title** | Business Requirements Document for Sapiens User Management System |
| **Version** | 2.0 |
| **Date** | November 19, 2025 |
| **Author** | Farid Hidayat (CEO, StartApp) |
| **Contributors** | Development Team, Security Team, Compliance Officers |
| **Approval Status** | Final Draft - Awaiting Stakeholder Sign-off |
| **Classification** | Internal - Confidential |
| **Distribution** | Project Team, Stakeholders, Management |

### 1.2 Revision History

| Version | Date | Description | Author | Approved By |
|---------|------|-------------|--------|-------------|
| 1.0 | 2025-11-19 | Initial draft based on entity and capability details | Grok (xAI) | - |
| 2.0 | 2025-11-19 | Comprehensive expansion with detailed schema, API specs, and technical documentation | Farid Hidayat | Pending |

### 1.3 Document Purpose

This Business Requirements Document (BRD) serves as the authoritative specification for the Sapiens User Management System (UMS). It provides:

1. **Business Justification**: Clear articulation of business needs and expected value
2. **Functional Specifications**: Detailed requirements for all system capabilities
3. **Technical Specifications**: Complete database schema, API definitions, and integration points
4. **Implementation Guidance**: Detailed specifications for development teams
5. **Success Metrics**: Measurable criteria for project success
6. **Risk Management**: Identified risks and mitigation strategies

This document is intended for:
- **Business Stakeholders**: Understanding business value and ROI
- **Development Teams**: Technical implementation guidance
- **QA/Testing Teams**: Acceptance criteria and test specifications
- **Security Teams**: Security requirements and compliance standards
- **Project Managers**: Timeline, budget, and resource planning

---

## 2. Project Overview

### 2.1 Executive Summary

The **Sapiens User Management System (UMS)** is a foundational, enterprise-grade microservice designed to handle the complete lifecycle of user accounts in a scalable, secure, and compliant manner. Built on Domain-Driven Design (DDD) principles and Role-Based Access Control (RBAC) architecture, Sapiens provides comprehensive user identity management for modern applications.

**Key Capabilities:**
- **User Provisioning**: Complete CRUD operations for user account management
- **Authentication**: Multi-factor authentication (MFA), OAuth, social login integration
- **Authorization**: Granular RBAC with roles, permissions, and hierarchical access control
- **Auditing & Logging**: Comprehensive event tracking for compliance and security monitoring
- **Password Management**: Self-service resets, strength policies, and Argon2id hashing
- **Session Management**: JWT-based token handling with expiry and refresh capabilities

**Technology Foundation:**
- **Backend**: Rust (Actix-Web framework) for performance and safety
- **Database**: MongoDB (primary) for scalability and flexibility
- **Architecture**: Microservices with clean architecture layers (Domain, Application, Infrastructure, Presentation)
- **Protocol**: gRPC (Protocol Buffers) for internal service communication, REST for external APIs
- **Security**: OWASP Top 10 compliant, GDPR/CCPA aligned

### 2.2 Project Name & Branding

**Official Name**: Sapiens - User Management System (UMS)

**Service Codename**: `sapiens`

**Branding Rationale**: The name "Sapiens" (Latin for "wise") reflects the intelligent, thoughtful approach to user management, emphasizing security wisdom and best practices.

### 2.3 Project Summary

**Problem Being Solved:**
Current user management across modern applications relies on fragmented tools and manual processes for authentication, role assignments, and auditing. This fragmentation leads to:
- Security vulnerabilities from inconsistent implementations
- High administrative burden from manual role/permission management
- Compliance risks due to inadequate audit trails
- Scalability challenges as user bases grow
- Integration complexity when connecting multiple systems

**Solution Provided:**
Sapiens provides a centralized, production-ready user management platform that:
- **Unifies Identity Management**: Single source of truth for all user data
- **Automates Access Control**: RBAC-based automated role and permission assignment
- **Ensures Compliance**: Built-in audit logging and GDPR/CCPA compliance
- **Scales Effortlessly**: Designed to handle 1M+ users with <100ms response times
- **Integrates Seamlessly**: RESTful APIs and gRPC services for easy integration

**Beneficiaries:**
1. **Application Administrators**: Streamlined user provisioning and role management
2. **Developers**: Easy-to-integrate APIs with comprehensive documentation
3. **End Users**: Frictionless authentication and self-service capabilities
4. **Security Teams**: Comprehensive audit trails and security controls
5. **Compliance Officers**: Built-in compliance features and reporting

### 2.4 Business Goals & Objectives

#### Primary Objectives (Must-Have)

1. **Security & Compliance**
   - Achieve 100% compliance with GDPR/CCPA data protection standards
   - Implement OWASP Top 10 security best practices
   - Zero high-severity security vulnerabilities in production
   - Complete security audit pass rate: 100%

2. **Performance & Scalability**
   - Support 1 million active user identities by Q2 2026
   - Authentication response time: <100ms for 99% of requests
   - Authorization checks: <200ms for complex permission queries
   - Handle 10,000 concurrent users without degradation

3. **Operational Efficiency**
   - Reduce administrative overhead by 40% through automation
   - Decrease user onboarding time by 50% (manual → automated)
   - Minimize support tickets by 30% through self-service features
   - Free up 40% of IT staff time for innovation vs. maintenance

4. **Reliability & Availability**
   - Achieve 99.9% system uptime (maximum 43.2 minutes downtime/month)
   - Recovery Time Objective (RTO): <5 minutes
   - Recovery Point Objective (RPO): <15 minutes
   - Zero-downtime deployments for updates

#### Secondary Objectives (Nice-to-Have)

1. **User Experience**
   - Minimize user friction in onboarding and login processes
   - Provide intuitive self-service portals
   - Support social login (OAuth) for convenience
   - Enable MFA enrollment during first login

2. **Integration Capabilities**
   - Integrate with at least 3 third-party identity providers (Auth0, Okta, Google OAuth)
   - Provide comprehensive RESTful APIs for external systems
   - Support email/SMS notification services (SendGrid, Twilio)
   - Export audit logs to monitoring tools (ELK Stack, Splunk)

3. **Advanced Features (Future Phases)**
   - AI-driven anomaly detection for suspicious login patterns
   - Advanced SAML federation support
   - Hardware-based MFA (YubiKey)
   - Mobile-specific push notifications

### 2.5 Business Value Proposition

#### Quantifiable Benefits

| Metric | Current State | Target State | Improvement | Timeline |
|--------|---------------|--------------|-------------|----------|
| Admin Overhead | 100% (baseline) | 60% | 40% reduction | 6 months |
| User Onboarding Time | 15 min (manual) | 7.5 min (automated) | 50% faster | 3 months |
| Security Incidents | Variable | Zero high-severity | 100% improvement | Ongoing |
| System Uptime | 95% (legacy) | 99.9% | +4.9% | 3 months |
| Compliance Audit Pass Rate | 70% | 100% | +30% | 6 months |
| User Satisfaction (NPS) | 50 | >70 | +40% | 6 months |

#### Financial Impact

**Cost Savings:**
- **Reduced Admin Labor**: $50,000/year (40% time savings × 2 FTE admins)
- **Avoided Security Breaches**: $150,000/year (estimated breach cost × reduced risk)
- **Compliance Fine Avoidance**: $500,000+ (GDPR fines for non-compliance)
- **Infrastructure Efficiency**: $20,000/year (optimized resource usage)

**Total Annual Savings**: $720,000+

**Investment Required**: $103,500 (one-time)

**ROI**: 594% in Year 1

**Payback Period**: ~1.7 months

#### Strategic Benefits

1. **Competitive Advantage**: Enterprise-grade security attracts larger clients
2. **Scalability Foundation**: Platform ready for 10x growth
3. **Regulatory Readiness**: Pre-built compliance for global markets
4. **Developer Productivity**: Reusable authentication/authorization platform
5. **Brand Protection**: Reduced risk of data breaches damaging reputation

### 2.6 Scope Definition

#### In-Scope Items ✅

**Core Entities:**
- Users (identity, credentials, profile)
- Roles (access groups)
- Permissions (granular actions)
- UserRoles (many-to-many junction)
- RolePermissions (many-to-many junction)
- UserSettings (user preferences)
- AuditLogs (event tracking)

**Functional Capabilities:**
- User provisioning (Create, Read, Update, Delete/Suspend)
- Authentication (email/password, MFA, OAuth social logins)
- Authorization (RBAC enforcement, permission checks)
- Auditing/Logging (event tracking, compliance reporting)
- Password management (self-service resets, strength policies, Argon2id hashing)
- Session management (JWT tokens, refresh, revocation)

**Integration Points:**
- Email/SMS services (SendGrid, Twilio)
- Identity providers (Auth0, Okta, Google OAuth)
- Monitoring tools (Prometheus, Grafana)
- Log aggregation (ELK Stack, Splunk)

**User Interfaces:**
- Admin dashboard (role/permission management)
- Self-service portal (profile updates, password resets)
- API documentation (OpenAPI/Swagger)

**Technical Components:**
- RESTful APIs (external integration)
- gRPC services (internal microservices)
- Database (MongoDB primary)
- Caching layer (Redis for sessions)
- Message queue (RabbitMQ for async events)

#### Out-of-Scope Items ❌

**Advanced Features (Future Phases):**
- SAML 2.0 federation (beyond basic OAuth 2.0)
- AI-driven anomaly detection and behavioral analytics
- Hardware-based MFA (YubiKey, smart cards)
- Biometric authentication (fingerprint, face recognition)
- Mobile-specific push notifications (handled by separate notification service)
- Advanced identity proofing (KYC/AML integration)
- Blockchain-based identity verification

**External Systems (Separate Services):**
- Email template design (handled by Postman service)
- File storage for profile pictures (handled by Bucket service)
- API gateway (handled by Rusty service)
- Business intelligence/reporting (separate BI platform)

**Non-Functional Exclusions:**
- Multi-tenancy (single tenant MVP)
- White-label customization (standard branding)
- Custom workflow engines (fixed RBAC workflow)

### 2.7 Project Constraints

#### Budget Constraints
- **Maximum Budget**: $103,500 (personnel + infrastructure + contingency)
- **Open-Source Tools Only**: No paid SaaS tiers (Auth0 premium, etc.) in MVP
- **Cloud Costs**: Limited to $500/month for Year 1 hosting

#### Timeline Constraints
- **MVP Delivery**: 3 months (15 weeks) from project kickoff
- **Production Launch**: Maximum 4 months including testing
- **No Timeline Extensions**: Fixed deadline for business reasons

#### Technical Constraints
- **No Runtime Package Installation**: Pre-configured libraries only (air-gapped environment)
- **MongoDB Primary Database**: Must use MongoDB (not PostgreSQL) per architecture decision
- **Rust Backend**: Required for performance and safety guarantees
- **Microservices Architecture**: Must integrate with existing monorepo structure

#### Resource Constraints
- **Team Size**: Maximum 5.75 FTE (full-time equivalent)
- **DevOps Support**: Limited to 0.5 FTE (shared resource)
- **Security Reviews**: Maximum 2 formal security audits

#### Regulatory Constraints
- **GDPR Compliance**: Must be compliant before production launch
- **CCPA Compliance**: Required for US market operations
- **OWASP Standards**: Must pass OWASP Top 10 security assessment
- **Data Residency**: Must support data residency requirements (multi-region)

---

## 3. Business Context

### 3.1 Current State Analysis

#### Existing System Challenges

**1. Fragmented Identity Management**
- **Issue**: User identities scattered across 5+ separate systems
- **Impact**: Duplicate accounts, inconsistent user data, synchronization failures
- **Cost**: 15+ hours/week spent on manual reconciliation
- **Risk**: Security gaps from out-of-sync permission states

**2. Manual Access Control Processes**
- **Issue**: Role assignments require manual intervention by IT staff
- **Impact**: 24-48 hour delay for new user access provisioning
- **Cost**: 30% of IT time spent on access requests
- **Risk**: Over-privileging due to lack of time for proper review

**3. Inadequate Audit Trails**
- **Issue**: Incomplete or scattered audit logs across systems
- **Impact**: Cannot demonstrate compliance with GDPR/CCPA requirements
- **Cost**: $50,000+ spent on manual compliance reporting annually
- **Risk**: Potential regulatory fines up to $20M for GDPR violations

**4. Weak Password Management**
- **Issue**: Inconsistent password policies, plain-text storage in legacy systems
- **Impact**: 40% of security incidents related to credential compromise
- **Cost**: Average $4.5M per data breach (IBM 2025 Cost of Breach Report)
- **Risk**: Reputation damage, customer trust erosion

**5. Limited Scalability**
- **Issue**: Current systems cannot handle projected 10x user growth
- **Impact**: Performance degradation at 50,000+ users
- **Cost**: Urgent infrastructure upgrades required ($200K+)
- **Risk**: Service outages during peak usage

#### Pain Points by Stakeholder

**System Administrators:**
- Spending 60% of time on repetitive provisioning tasks
- No centralized view of user access across systems
- Manual role assignment prone to human error
- Difficulty auditing who has access to what

**Developers:**
- Each application requires custom authentication code
- Inconsistent API patterns across services
- No standardized way to check user permissions
- Integration testing complicated by auth complexity

**End Users:**
- Multiple username/password combinations to remember
- Frequent account lockouts from complex policies
- Long wait times for access requests (24-48 hours)
- No self-service password reset capability

**Security Team:**
- Cannot quickly identify security posture of accounts
- No real-time anomaly detection
- Difficult to enforce consistent password policies
- Incomplete visibility into access patterns

**Compliance Officers:**
- Manual effort to generate audit reports
- Cannot prove continuous compliance
- Gaps in data retention policies
- Difficult to demonstrate right-to-deletion (GDPR)

### 3.2 Problem Statement

**Executive Summary:**
Current user management infrastructure relies on disparate tools and manual processes for authentication, role assignments, and auditing. This fragmentation creates security vulnerabilities, inconsistent access controls, high administrative burden, and compliance risks. The business faces increased operational costs, reduced scalability for growing user bases, and exposure to regulatory fines up to $20M. This situation is urgent due to rising cyber threats, expanding user volumes (projected 10x growth in 2 years), and upcoming GDPR/CCPA compliance audits in Q2 2026.

**Specific Problems:**

1. **Security Vulnerabilities**
   - 40% of incidents tied to weak password management
   - Inconsistent authentication implementations across services
   - Delayed permission revocations (avg 48 hours)
   - No MFA enforcement capability

2. **Administrative Burden**
   - 30% of IT time spent on manual access provisioning
   - 15+ hours/week on user data reconciliation
   - Complex role management without tools
   - No automated workflow for role assignments

3. **Compliance Risks**
   - Incomplete audit trails for user actions
   - Cannot demonstrate GDPR right-to-deletion
   - No data retention automation
   - Manual compliance reporting ($50K/year effort)

4. **Scalability Limitations**
   - Performance degradation at 50,000+ users
   - Cannot support projected 1M users by 2026
   - No horizontal scaling capability
   - Infrastructure upgrades needed ($200K+)

5. **Integration Complexity**
   - Each service implements custom auth logic
   - Inconsistent API patterns
   - Difficult to add new applications
   - High development effort for identity features

**Business Impact:**
- **Financial**: $350K+ annual losses (labor, inefficiency, breach risk)
- **Operational**: 40% of IT capacity consumed by identity management
- **Strategic**: Cannot pursue enterprise clients due to security concerns
- **Regulatory**: Exposure to $20M+ in potential GDPR/CCPA fines
- **Reputation**: Customer trust at risk from security incidents

**Urgency Drivers:**
- Q2 2026 compliance audit deadline
- Projected 10x user growth in 24 months
- Recent security incidents highlighting vulnerabilities
- Enterprise client requirements for SOC 2 compliance
- Competitive pressure from better-secured alternatives

### 3.3 Solution Overview

Sapiens provides a **centralized, production-ready user management platform** that eliminates fragmentation through:

#### Core Solution Components

**1. Unified Identity Management**
- Single source of truth for all user data
- Centralized user profile with extensible attributes
- Automated synchronization across integrated systems
- Real-time identity state management

**2. Automated Access Control (RBAC)**
- Role-based access control with hierarchical permissions
- Self-service role request workflows with approval chains
- Automated role assignment based on user attributes
- Granular permission model (resource:action format)

**3. Enterprise-Grade Authentication**
- Multi-factor authentication (TOTP, SMS)
- OAuth 2.0 social login (Google, GitHub, Microsoft)
- Passwordless authentication options
- Session management with JWT tokens

**4. Comprehensive Auditing**
- Immutable audit logs for all user events
- Real-time compliance reporting dashboards
- Automated data retention policies
- Export capabilities for SIEM integration

**5. Self-Service Capabilities**
- User profile management
- Password reset without admin intervention
- MFA device enrollment
- Personal settings and preferences

**6. Developer-Friendly APIs**
- RESTful APIs for external integration
- gRPC services for internal microservices
- Comprehensive OpenAPI documentation
- SDK libraries for common languages

#### Technical Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    External Clients                      │
│         (Web Apps, Mobile Apps, Third-Party)             │
└─────────────────────┬───────────────────────────────────┘
                      │
          ┌──────────REST API (HTTPS)──────────┐
          │                                      │
┌─────────▼────────────────────────────────────▼─────────┐
│              Sapiens User Management System              │
│  ┌──────────────────────────────────────────────────┐  │
│  │         Presentation Layer (Actix-Web)            │  │
│  │  - REST Controllers  - gRPC Handlers              │  │
│  │  - Request Validation  - Response Mapping          │  │
│  └──────────────────┬───────────────────────────────┘  │
│                     │                                    │
│  ┌──────────────────▼───────────────────────────────┐  │
│  │         Application Layer (Use Cases)             │  │
│  │  - User Provisioning  - Authentication            │  │
│  │  - Authorization  - Password Management           │  │
│  └──────────────────┬───────────────────────────────┘  │
│                     │                                    │
│  ┌──────────────────▼───────────────────────────────┐  │
│  │      Domain Layer (Business Logic - Proto)        │  │
│  │  - Entities  - Value Objects  - Domain Services   │  │
│  │  - Repositories  - Events  - Specifications       │  │
│  └──────────────────┬───────────────────────────────┘  │
│                     │                                    │
│  ┌──────────────────▼───────────────────────────────┐  │
│  │       Infrastructure Layer (Persistence)          │  │
│  │  - MongoDB Repos  - Redis Cache  - Event Bus      │  │
│  └──────────────────┬───────────────────────────────┘  │
└────────────────────┼────────────────────────────────────┘
                     │
     ┌───────────────┼───────────────┐
     │               │               │
┌────▼─────┐  ┌─────▼──────┐  ┌────▼─────┐
│ MongoDB   │  │   Redis    │  │ RabbitMQ │
│ (Primary) │  │  (Cache)   │  │ (Events) │
└───────────┘  └────────────┘  └──────────┘
```

#### Key Features Delivered

| Category | Features |
|----------|----------|
| **User Provisioning** | Create, Read, Update, Delete/Suspend accounts<br/>Bulk import/export<br/>Default role assignment<br/>Email verification |
| **Authentication** | Email/password login<br/>Multi-factor authentication (MFA)<br/>OAuth 2.0 social login<br/>Session management (JWT)<br/>Account lockout (5 failed attempts) |
| **Authorization** | Role-based access control (RBAC)<br/>Hierarchical permissions<br/>Permission inheritance<br/>Deny-by-default security |
| **Auditing** | Immutable event logging<br/>User action tracking<br/>IP address logging<br/>Compliance reporting |
| **Password Mgmt** | Argon2id hashing<br/>Self-service reset<br/>Strength policies<br/>Password history (no reuse) |
| **Self-Service** | Profile updates<br/>Password changes<br/>MFA enrollment<br/>Settings management |

### 3.4 Business Value Proposition

#### Immediate Benefits (0-3 Months)

1. **Operational Efficiency**
   - 50% reduction in user onboarding time (15 min → 7.5 min)
   - 30% reduction in support tickets (self-service features)
   - 25% reduction in admin labor (automated workflows)
   - Zero-touch provisioning for standard roles

2. **Security Improvements**
   - Consistent Argon2id password hashing across all users
   - MFA enforcement for administrative accounts
   - Real-time account lockout on suspicious activity
   - Centralized session management and revocation

3. **Developer Productivity**
   - Standardized authentication/authorization APIs
   - Reduced integration time for new applications (4 weeks → 1 week)
   - Reusable identity platform across services
   - Comprehensive API documentation

#### Medium-Term Benefits (3-12 Months)

1. **Cost Savings**
   - $50,000/year saved on admin labor
   - $150,000/year avoided breach costs (risk reduction)
   - $50,000/year saved on manual compliance reporting
   - $20,000/year infrastructure optimization

2. **Compliance Achievement**
   - 100% GDPR/CCPA compliance demonstrated
   - Automated audit trails for regulatory review
   - Right-to-deletion capabilities
   - Data retention automation

3. **Scalability Foundation**
   - Support for 1M+ users without major rearchitecture
   - Horizontal scaling capability
   - <100ms authentication response times
   - 99.9% uptime SLA

#### Long-Term Strategic Value (12+ Months)

1. **Competitive Advantage**
   - Enterprise client acquisition enabled (SOC 2 readiness)
   - Differentiation through security posture
   - Faster time-to-market for new applications
   - Platform for AI/ML-driven security features

2. **Business Enablement**
   - Support for 10x user growth without proportional cost increase
   - Foundation for multi-region deployment
   - Reusable identity platform reduces future dev costs by 60%
   - Data-driven insights from audit logs

3. **Risk Reduction**
   - 50% reduction in security incident probability
   - $500K+ avoided in potential GDPR fines
   - Improved brand reputation and customer trust
   - Faster incident response through centralized logs

### 3.5 Market & Competitive Analysis

#### Industry Benchmarks

| Metric | Industry Average | Sapiens Target | Competitive Position |
|--------|------------------|----------------|----------------------|
| Auth Response Time | 250ms | <100ms | Top 10% |
| Uptime SLA | 99.5% | 99.9% | Top 25% |
| Password Hashing | Bcrypt | Argon2id | Leading Edge |
| MFA Adoption | 35% | 100% (admins) | Leading Edge |
| Audit Retention | 90 days | 1 year | Above Average |

#### Competitive Landscape

**Commercial Solutions:**
- **Auth0**: $240-$1,200/month, feature-rich but expensive for MVP
- **Okta**: Enterprise focus, $2-$15/user/month
- **AWS Cognito**: $0.05/MAU, lock-in to AWS ecosystem

**Open-Source Alternatives:**
- **Keycloak**: Java-based, heavy resource usage
- **Ory**: Go-based, good but less Rust integration
- **SuperTokens**: Node.js, limited Rust support

**Sapiens Differentiation:**
- ✅ **Rust-based**: Superior performance and safety guarantees
- ✅ **MongoDB-first**: Better fit for document-oriented user data
- ✅ **Schema-first**: Type-safe domain modeling with YAML schemas and code generation
- ✅ **Zero-cost**: Open-source, no per-user fees
- ✅ **Microservices-ready**: gRPC and REST APIs
- ✅ **DDD architecture**: Clean, maintainable, extensible

### 3.6 Success Metrics & KPIs

#### Business Metrics

| Metric | Baseline | Target (3mo) | Target (12mo) | Measurement Method |
|--------|----------|--------------|---------------|--------------------|
| Admin Overhead | 100% | 75% | 60% | Time tracking audit |
| Onboarding Time | 15 min | 10 min | 7.5 min | Process measurement |
| Support Tickets | 100/mo | 80/mo | 70/mo | Ticket system reports |
| Security Incidents | 5/year | 2/year | 0/year | Incident logs |
| Compliance Pass Rate | 70% | 90% | 100% | Audit results |

#### Technical Metrics

| Metric | Baseline | Target (3mo) | Target (12mo) | Measurement Method |
|--------|----------|--------------|---------------|--------------------|
| Auth Response Time | N/A | <150ms | <100ms | APM monitoring |
| System Uptime | 95% | 99.5% | 99.9% | Uptime monitoring |
| Concurrent Users | 5K | 10K | 50K | Load testing |
| API Availability | N/A | 99.5% | 99.9% | API monitoring |
| Test Coverage | N/A | 80% | 95% | Code coverage tools |

#### User Satisfaction Metrics

| Metric | Baseline | Target (3mo) | Target (12mo) | Measurement Method |
|--------|----------|--------------|---------------|--------------------|
| NPS Score | 50 | 60 | >70 | User surveys |
| Login Success Rate | 85% | 92% | 95% | Analytics |
| Self-Service Adoption | 0% | 40% | 60% | Feature usage analytics |
| Password Reset Time | 24hrs | 5min | <1min | Process tracking |

---

## 4. Stakeholders

### 4.1 Stakeholder Matrix

| Name/Role | Department | Responsibility | Involvement Level | Communication Frequency |
|-----------|------------|----------------|-------------------|------------------------|
| **Farid Hidayat** | CEO / Executive | Final approval, strategic direction, budget authorization | High - All phases | Weekly status, major milestones |
| **Technical Lead** (TBD) | Engineering | Architecture decisions, technical implementation, code reviews | High - Design & Development | Daily standups, bi-weekly reviews |
| **DevOps Manager** (TBD) | Infrastructure | CI/CD pipeline, deployment, infrastructure setup, monitoring | High - Integration & Deployment | Bi-weekly, deployment events |
| **Product Owner** (TBD) | Product Management | Requirements definition, UAT coordination, stakeholder liaison | High - Requirements & Testing | Daily during planning, weekly updates |
| **Security Engineer** (TBD) | Security | Security review, penetration testing, compliance verification | High - Security & Compliance | Weekly during dev, intensive during audits |
| **Compliance Officer** (TBD) | Legal/Compliance | GDPR/CCPA compliance review, audit support, policy approval | Medium - Reviews & Approvals | Bi-weekly reviews, audit milestones |
| **QA Engineer** (TBD) | Quality Assurance | Test planning, execution, automation, defect tracking | High - Testing phase | Daily during testing phase |
| **Frontend Developer** (TBD) | Engineering | Admin dashboard, self-service portal development | Medium - UI Development | As needed during sprint |
| **Backend Developers** (2 FTE) | Engineering | API development, business logic, database implementation | High - Core Development | Daily standups |
| **End-User Admins** (Representative) | Operations | UAT participation, training feedback, operational readiness | Medium - Testing & Training | UAT sessions, training sessions |
| **End Users** (Representatives) | Various | Feature feedback, usability testing, acceptance | Low - UAT only | UAT sessions only |

### 4.2 RACI Matrix

| Activity | CEO | Tech Lead | DevOps | Product Owner | Security | QA | Backend Dev | Frontend Dev |
|----------|-----|-----------|--------|---------------|----------|-------|-------------|--------------|
| **Requirements Definition** | A | C | I | R | C | I | C | C |
| **Architecture Design** | I | R/A | C | C | C | I | C | I |
| **Security Design** | I | C | C | C | R/A | C | C | I |
| **Backend Development** | I | A | I | C | C | I | R | I |
| **Frontend Development** | I | A | I | C | I | C | C | R |
| **Database Schema** | I | A | C | C | C | I | R | I |
| **API Documentation** | I | A | I | R | I | C | R | C |
| **Testing Strategy** | I | A | C | C | C | R | C | C |
| **Security Audit** | I | C | C | C | R/A | C | C | I |
| **Deployment** | I | A | R | C | C | C | C | I |
| **Go-Live Approval** | A | R | R | R | R | R | I | I |

**Legend**: R = Responsible, A = Accountable, C = Consulted, I = Informed

### 4.3 Detailed User Personas

#### Persona 1: System Administrator (Sarah Chen)

**Demographics:**
- Age: 32
- Role: Senior System Administrator
- Experience: 7 years in IT/Security operations
- Education: B.S. Computer Science, CISSP certified
- Organization: Medium enterprise (5,000+ employees)

**Technical Profile:**
- **Skill Level**: Advanced
- **Tools Used**: Active Directory, Okta, AWS IAM, PowerShell, Python
- **Proficiencies**: RBAC systems, LDAP, OAuth, security best practices
- **Learning Style**: Documentation-focused, prefers comprehensive guides

**Responsibilities:**
- Provision and deprovision user accounts (50+ requests/week)
- Assign and modify roles/permissions
- Monitor access control compliance
- Generate audit reports for quarterly compliance reviews
- Respond to security incidents involving user accounts
- Train junior admins on identity management procedures

**Goals & Motivations:**
- Automate repetitive provisioning tasks to focus on strategic security work
- Maintain 100% compliance with SOC 2 and GDPR requirements
- Reduce time-to-access for new employees (currently 48 hours → target <2 hours)
- Implement comprehensive audit trails for all user activities
- Prevent security breaches through proactive access reviews

**Pain Points & Frustrations:**
- Spends 60% of time on manual, repetitive tasks (provisioning, role changes)
- No centralized view of user access across 5+ disparate systems
- Manual role assignments prone to human error and over-privileging
- Difficult to prove compliance due to scattered audit logs
- Delayed permission updates create security windows of vulnerability
- Complex approval workflows require email chains and manual tracking

**User Journey (Current State):**
1. Receive access request via email/ticket system → 5 min
2. Verify approval from manager → 30 min (wait time)
3. Manually create account in 3-5 separate systems → 20 min
4. Assign roles based on job title/department → 10 min
5. Document changes in spreadsheet for audit → 5 min
6. Email user with credentials → 2 min
7. **Total Time**: 72 minutes (includes wait times)

**User Journey (Desired State with Sapiens):**
1. Receive access request via self-service portal → 0 min (automated)
2. Auto-approval for standard roles OR manager approval via portal → 5 min
3. Sapiens auto-provisions account with default roles → 0 min (automated)
4. Sapiens sends welcome email with password reset link → 0 min (automated)
5. Audit log automatically created → 0 min (automated)
6. **Total Time**: 5 minutes (90% reduction)

**Success Criteria:**
- Reduce provisioning time by 80%
- Centralized dashboard for all user access
- Automated audit trails for compliance
- Self-service role requests with approval workflows
- Real-time access reviews and anomaly detection

**Preferred Features:**
- Bulk user import/export
- Role templates for common job functions
- Advanced search and filtering
- Compliance report generation
- Email notifications for critical events
- API access for automation scripts

---

#### Persona 2: End-User Developer (Marcus Johnson)

**Demographics:**
- Age: 28
- Role: Full-Stack Developer
- Experience: 4 years in software development
- Education: B.S. Software Engineering
- Organization: Tech startup (200 employees)

**Technical Profile:**
- **Skill Level**: Intermediate to Advanced
- **Tools Used**: Git, Docker, VS Code, Postman, React, Node.js
- **Proficiencies**: REST APIs, OAuth 2.0, JWT, basic security concepts
- **Learning Style**: Hands-on, prefers code examples and interactive docs

**Responsibilities:**
- Develop and maintain web applications
- Integrate authentication/authorization into new features
- Test user flows including login, permissions, session management
- Debug auth-related issues in development and staging
- Review security best practices for user data handling

**Goals & Motivations:**
- Quickly integrate authentication into new applications (<1 week)
- Avoid reinventing auth logic for each project
- Ensure consistent security across all applications
- Focus on business logic rather than identity management plumbing
- Learn modern auth patterns (OAuth, MFA) through production use

**Pain Points & Frustrations:**
- Each application requires custom authentication code (2-3 weeks of dev time)
- Inconsistent API patterns across different identity systems
- Difficult to test auth flows in local development
- No standardized way to check user permissions in code
- Complex integration testing when auth is involved
- Frequent lockouts during development/testing (password policy too strict)
- Lack of clear documentation and code examples

**User Journey (Current State - Integrating Auth):**
1. Research auth libraries and patterns → 4 hours
2. Set up local auth server/database → 3 hours
3. Implement user registration endpoint → 6 hours
4. Implement login endpoint with JWT → 4 hours
5. Implement password reset flow → 4 hours
6. Add permission checks to protected routes → 8 hours
7. Write tests for auth flows → 8 hours
8. Debug edge cases and security issues → 8 hours
9. **Total Time**: ~45 hours (~1.5 weeks)

**User Journey (Desired State with Sapiens):**
1. Read Sapiens API documentation → 1 hour
2. Install Sapiens SDK library → 15 min
3. Configure API endpoint and credentials → 15 min
4. Implement login using SDK (3 lines of code) → 30 min
5. Implement permission checks using middleware → 1 hour
6. Test using Sapiens sandbox environment → 2 hours
7. **Total Time**: ~5.5 hours (88% reduction)

**Success Criteria:**
- Comprehensive, easy-to-follow API documentation
- SDK libraries for popular languages (JavaScript, Python, Rust)
- Interactive API playground for testing
- Code examples for common use cases
- Development sandbox environment
- Minimal integration time (<1 day)

**Preferred Features:**
- RESTful API with consistent patterns
- OAuth 2.0 social login support
- Webhooks for user events (created, updated, deleted)
- JWT-based session management
- Sandbox environment with test users
- Detailed error messages with troubleshooting guidance

---

#### Persona 3: Compliance Officer (Patricia Rodriguez)

**Demographics:**
- Age: 45
- Role: Senior Compliance Manager
- Experience: 15 years in legal/compliance, 8 years in data privacy
- Education: J.D. Law, CIPP/E certified
- Organization: Enterprise (10,000+ employees)

**Technical Profile:**
- **Skill Level**: Business user, basic technical understanding
- **Tools Used**: Excel, compliance management software, audit platforms
- **Proficiencies**: GDPR, CCPA, SOC 2, audit processes, risk assessment
- **Learning Style**: Visual dashboards, reports, process documentation

**Responsibilities:**
- Ensure GDPR/CCPA compliance for user data handling
- Coordinate quarterly compliance audits
- Respond to data subject access requests (DSARs)
- Maintain audit trails for regulatory review
- Assess risk of data breaches and privacy violations
- Train staff on compliance requirements

**Goals & Motivations:**
- Demonstrate continuous compliance to auditors
- Respond to DSARs within 30-day legal requirement
- Minimize risk of regulatory fines ($20M+ for GDPR violations)
- Automate compliance reporting to save time
- Prove data retention and deletion policies are enforced

**Pain Points & Frustrations:**
- Manual effort to generate audit reports ($50K+/year in labor)
- Cannot prove continuous compliance, only point-in-time snapshots
- Difficult to demonstrate right-to-deletion (GDPR Article 17)
- Incomplete audit logs across multiple systems
- Time-consuming DSARs require manual data collection from 5+ systems
- Gaps in data retention policies create compliance risk

**User Journey (Current State - DSAR Response):**
1. Receive DSAR request from user → Day 1
2. Manually identify user across 5+ systems → 2 days
3. Export data from each system → 3 days
4. Manually combine and redact data → 2 days
5. Legal review → 3 days
6. Send response to user → Day 11
7. **Total Time**: 11 days (out of 30-day deadline)

**User Journey (Desired State with Sapiens):**
1. Receive DSAR request → Day 1
2. Search user in Sapiens portal → 5 min
3. Export all user data in one click → 5 min
4. Automated redaction of sensitive fields → 0 min
5. Legal review → 1 day
6. Send response → Day 2
7. **Total Time**: 2 days (82% reduction)

**Success Criteria:**
- Automated audit trail generation
- One-click DSAR data export
- Automated data retention enforcement
- Compliance dashboard with real-time status
- Historical audit logs for 1+ year retention
- Proof of deletion for right-to-be-forgotten requests

**Preferred Features:**
- Compliance dashboard with visual KPIs
- Automated audit reports (PDF/CSV export)
- DSAR workflow automation
- Data retention policy configuration
- Immutable audit logs
- Anonymization for GDPR compliance

---

#### Persona 4: End User (General Employee - Alex Kim)

**Demographics:**
- Age: 35
- Role: Marketing Manager
- Experience: Non-technical business user
- Education: B.A. Marketing
- Organization: Any size

**Technical Profile:**
- **Skill Level**: Basic user
- **Tools Used**: Email, Office suite, web browsers, mobile apps
- **Proficiencies**: Basic computer literacy, email, web forms
- **Learning Style**: Visual, guided wizards, minimal text

**Responsibilities:**
- Access company applications for daily work
- Maintain own user profile information
- Manage personal settings and preferences
- Occasionally request access to new systems

**Goals & Motivations:**
- Quick, easy login to applications without friction
- Self-service password reset without IT help
- Remember one password instead of multiple
- Access applications from any device securely
- Maintain privacy and control over personal data

**Pain Points & Frustrations:**
- Multiple username/password combinations to remember (5+ systems)
- Frequent account lockouts from password policies
- Long wait times for access requests (24-48 hours)
- No self-service password reset (must call IT help desk)
- Complex password requirements hard to remember
- Can't update own profile information (email, phone, etc.)

**User Journey (Current State - Password Reset):**
1. Forget password, attempt login 3x → 5 min
2. Account locked, call IT help desk → 10 min (wait time)
3. Verify identity with help desk agent → 5 min
4. IT manually resets password → 5 min
5. Receive temporary password via email → 5 min (wait)
6. Login with temp password, forced to change → 3 min
7. **Total Time**: 33 minutes of productivity loss

**User Journey (Desired State with Sapiens):**
1. Forget password, click "Forgot Password" → 30 sec
2. Receive reset email with link → 1 min
3. Click link, enter new password → 1 min
4. Login successfully → 30 sec
5. **Total Time**: 3 minutes (91% reduction)

**Success Criteria:**
- Simple, intuitive login page
- Self-service password reset (<5 minutes)
- Option for social login (Google, Microsoft)
- MFA setup wizard with clear instructions
- Profile self-service for non-sensitive fields
- "Remember me" option for trusted devices

**Preferred Features:**
- Single Sign-On (SSO) across applications
- Social login (Google, Microsoft, LinkedIn)
- Simple MFA (authenticator app or SMS)
- Self-service profile updates
- Password strength indicator during reset
- Clear, friendly error messages

---

## 4. Functional Requirements

### 4.1 Core Features
```
Feature 1: User Provisioning
- As a system administrator, I want to create new user accounts with default roles, so that onboarding is automated and consistent
- As a system administrator, I want to update user profiles and suspend accounts, so that I can manage lifecycle changes securely
- As an end-user, I want to edit my own non-sensitive profile details, so that my information stays current

Feature 2: Authentication & Password Management
- As an end-user, I want to login with email/password and MFA, so that my identity is verified securely
- As an end-user, I want to reset my password via email link, so that I can recover access without admin help
- As a system administrator, I want to enforce password policies and track resets, so that security standards are maintained

Feature 3: Authorization & Auditing
- As a system administrator, I want to assign roles and permissions to users, so that access is granular and auditable
- As an end-user, I want my requests to be authorized based on roles, so that I only access permitted resources
- As a compliance officer, I want to view audit logs of user actions, so that I can ensure regulatory compliance
```

### 4.2 Detailed Requirements
```
FR-001: User Account Creation
- The system shall validate email uniqueness and hash passwords (Argon2) before storage
- The system shall auto-assign a default 'user' role via UserRoles junction
- The system shall send confirmation email upon creation
- The system shall support self-registration with admin approval workflow

FR-002: Role and Permission Assignment
- The system shall enforce many-to-many relationships via UserRoles and RolePermissions junctions
- The system shall cascade permission checks for effective user access
- The system shall log all assignments with timestamps and IP addresses
- The system shall prevent assignments without 'assign_role' permission

FR-003: Auditing and Logging
- The system shall create immutable entries in AuditLogs for events like logins and changes
- The system shall retain logs for 1 year and support queries by user/date/action
- The system shall alert on anomalies (e.g., failed logins >5)
- The system shall mask sensitive data in logs per GDPR
```

---

## 5. Non-Functional Requirements

### 5.1 Performance Requirements
```
PERF-001: Response Time
- Authentication and authorization checks shall complete in <200ms for 99% of requests
- Permission queries (with joins) shall return in <100ms
- Audit log searches shall handle 10k entries in <1 second

PERF-002: Throughput
- System shall support 10,000 concurrent users with JWT token handling
- Process 50,000 user sessions per hour without degradation
- Scale to 1 million user records with sharding/indexing

PERF-003: Availability
- System shall maintain 99.9% uptime with automated backups
- Recover from failures within 5 minutes via redundancy
- Handle peak loads (e.g., mass onboarding) without downtime
```

### 5.2 Security Requirements
```
SEC-001: Authentication
- Support MFA (TOTP) and OAuth; lock after 5 failed attempts (15-min unlock)
- Hash passwords with Argon2id (min 60 chars); enforce 12-char min with complexity
- JWT sessions expire in 24 hours with refresh tokens

SEC-002: Authorization
- Implement RBAC with deny-by-default; check via middleware on all requests
- Support hierarchical roles (e.g., admin inherits editor perms)
- Log all denied accesses for review

SEC-003: Data Protection
- Encrypt sensitive fields (e.g., passwords) at rest with AES-256
- Use HTTPS/TLS for all transmissions; comply with OWASP Top 10
- Implement RLS in DB; anonymize PII in logs for GDPR/CCPA
```

### 5.3 Usability Requirements
```
USA-001: User Interface
- Admin dashboard and self-service portals shall be responsive across devices
- New admins shall complete user provisioning in <5 minutes via intuitive forms
- Comply with WCAG 2.1 AA for accessibility (e.g., screen reader support)

USA-002: User Experience
- Provide contextual error messages (e.g., "Invalid MFA code—resend?") with fixes
- Require confirmations for sensitive actions (e.g., role deletion)
- Persist user settings (e.g., theme) via UserSettings entity
```

---

## 6. User Stories & Use Cases

### 6.1 Primary Use Cases
```
Use Case 1: User Onboarding
Actor: System Administrator
Description: Create and provision a new user account
Preconditions: Admin is logged in with provisioning permissions
Main Flow:
1. Admin navigates to "Add User" in dashboard
2. System displays form with fields (email, password, name, default role)
3. Admin fills and submits form
4. System validates uniqueness, hashes password, assigns role
5. System sends confirmation email and logs event
6. System redirects to user profile view
Postconditions: User is active; audit log entry created

Use Case 2: Role Assignment
Actor: System Administrator
Description: Assign a role to an existing user
Preconditions: Admin is logged in; user exists
Main Flow:
1. Admin searches for user and selects "Manage Roles"
2. System displays current roles and available options
3. Admin selects role and confirms
4. System updates UserRoles junction and cascades permissions
5. System logs assignment and notifies user if configured
Postconditions: User's effective permissions updated; access enforced immediately
```

### 6.2 User Journey Mapping
```
Admin Journey:
1. Login with MFA → 2. Dashboard Overview → 3. User Search/Provisioning → 4. Role Assignment → 5. Audit Review → 6. Logout

End-User Journey:
1. Login via Email/OAuth → 2. Profile/Settings Access → 3. Resource Request/Authorization Check → 4. Password Reset (if needed) → 5. Logout

Compliance Officer Journey:
1. Login → 2. Audit Logs Query → 3. Anomaly Alert Review → 4. Report Export → 5. Compliance Dashboard → 6. Logout
```

---

## 7. Data Requirements

### 7.1 Data Entities
```
Entity: Users
Attributes:
- id (UUID, Primary Key, Required)
- username (VARCHAR 50, Unique, Required, Min 3/Max 50, Alphanumeric + _/-)
- email (VARCHAR 255, Unique, Required, Valid Email Regex, Lowercase)
- password_hash (TEXT, Required, Min 60 for Argon2)
- first_name (VARCHAR 100, Required, Max 100, Letters + spaces/apostrophes)
- last_name (VARCHAR 100, Required, Max 100, Letters + spaces/apostrophes)
- status (ENUM: active/inactive/suspended, Required, Default active)
- created_at (TIMESTAMP, Required, Auto UTC)
- updated_at (TIMESTAMP, Required, Auto UTC, Trigger Update)
- last_login (TIMESTAMP, Optional)
- phone_number (VARCHAR 20, Optional, E.164 Regex)
- profile_picture_url (VARCHAR 500, Optional, Valid URL)

Entity: Roles
Attributes:
- id (UUID, Primary Key, Required)
- name (VARCHAR 50, Unique, Required, Max 50, Letters + _)
- description (TEXT, Optional, Max 1000)
- created_at (TIMESTAMP, Required, Auto UTC)
- updated_at (TIMESTAMP, Required, Auto UTC, Trigger Update)

Entity: Permissions
Attributes:
- id (UUID, Primary Key, Required)
- name (VARCHAR 100, Unique, Required, Max 100, action:resource Format)
- description (TEXT, Optional, Max 500)
- resource (VARCHAR 50, Optional, Max 50)
- created_at (TIMESTAMP, Required, Auto UTC)
- updated_at (TIMESTAMP, Required, Auto UTC, Trigger Update)

Entity: UserPermissions (Direct Permission Grants)
Attributes:
- id (UUID, Primary Key, Required)
- user_id (UUID, Foreign Key to Users, Required)
- permission_id (String, Foreign Key to Permissions, Required)
- granted_at (TIMESTAMP, Required, Auto UTC)
- granted_by (UUID, Foreign Key to Users, Required)
- expires_at (TIMESTAMP, Optional, for temporary grants)
- reason (TEXT, Required, Max 500, explanation for direct grant)
- resource_id (String, Optional, for resource-specific permissions)
- resource_type (String, Optional, type of resource e.g., "post", "project")
- is_active (Boolean, Required, Default true)
- revoked_at (TIMESTAMP, Optional)
- revoked_by (UUID, Foreign Key to Users, Optional)
- revoked_reason (TEXT, Optional, Max 500)
```

### 7.2 Data Relationships
```
Relationships:
- Users (1) to (N) UserRoles: One user can have multiple roles (many-to-many junction)
- Roles (1) to (N) UserRoles: One role can be assigned to multiple users
- Roles (1) to (N) RolePermissions: One role can have multiple permissions (many-to-many junction)
- Permissions (1) to (N) RolePermissions: One permission can apply to multiple roles
- Users (1) to (N) UserPermissions: One user can have multiple direct permissions (NEW - Hybrid RBAC)
- Permissions (1) to (N) UserPermissions: One permission can be granted directly to multiple users (NEW)
- Users (1) to (1) UserSettings: One user has one settings record (one-to-one)
- Users (1) to (N) AuditLogs: One user generates multiple log entries (one-to-many)

Permission Resolution Model (Hybrid RBAC):
- Effective Permissions = Role-Based Permissions ∪ Direct User Permissions
- Direct permissions can override role restrictions
- Direct permissions can be scoped to specific resources
- Direct permissions can have expiry dates for temporary access
```

### 7.3 Data Migration Requirements
```
Migration Requirements:
- Import legacy user records (up to 100k) from CSV/JSON sources, mapping to Users schema
- Hash existing plain-text passwords during import; flag for reset
- Migrate role/permission mappings to junctions, resolving duplicates via unique keys
- Preserve timestamps and status; validate emails for uniqueness
- Generate migration report with error counts and success rate (>95%)
- Support dry-run mode and rollback for partial failures
```

---

## 8. Integration Requirements

### 8.1 External Systems
```
Integration 1: Email/SMS Service (e.g., SendGrid/Twilio)
Purpose: Handle notifications for confirmations, resets, and alerts
Requirements:
- Send templated emails/SMS with tokens (e.g., reset links expiring in 1 hour)
- Track delivery/bounces; support unsubscribe for compliance
- API rate-limited to 100/min; fallback to in-app notifications

Integration 2: Identity Providers (e.g., Auth0, Okta, Google OAuth)
Purpose: Enable federated authentication and social logins
Requirements:
- OAuth 2.0/JWT token exchange for seamless login
- Sync user profiles on first login; handle token revocation
- Support MFA passthrough; log external auth events

Integration 3: Monitoring Tools (e.g., ELK Stack/Splunk)
Purpose: Export audit logs for advanced analysis
Requirements:
- Real-time streaming of AuditLogs via API/webhooks
- Filter by severity/date; retain 1 year per compliance
- Secure transmission with encryption
```

### 8.2 API Requirements
```
API Requirements:
- RESTful endpoints under /api/v1/user with JSON payloads/responses
- JWT/OAuth 2.0 auth; rate limit 100 req/min per IP/user
- OpenAPI/Swagger docs for all endpoints (e.g., POST /users, GET /permissions)
- Versioning (v1 base); support pagination for lists (e.g., users?limit=50)
- Standard errors: HTTP 4xx/5xx with {code: 'VALIDATION_ERROR', message: '...'}
- Idempotency via request IDs for creates/updates
```

---

## 9. Technical Constraints

### 9.1 Technology Stack
```
Frontend:
- Framework: React.js with TypeScript
- State Management: Zustand
- UI Library: Material-UI
- Testing: Jest, React Testing Library

Backend:
- Language: Rust preferred for perf
- Framework: Express/FastAPI
- Database: MongoDB (NoSQL alt) first choice, PostgreSQL (primary)
- ORM: Prisma/SQLAlchemy; Crypto: Argon2 for hashing

Infrastructure:
- Cloud: VPS
- Container: Docker/Kubernetes
- CI/CD: GitHub Actions/Jenkins
- Monitoring: Prometheus + Grafana; Logging: ELK
```

### 9.2 Technical Limitations
```
Constraints:
- Prefer relational DB for joins; NoSQL only for high-read denormalization
- Open-source only (no paid tiers like Auth0 premium initially)
- Comply with OWASP/GDPR; no plain-text storage
- Support microservices architecture with HTTPS/JWT
- WCAG 2.1 AA accessibility; multi-language (English primary)
- No direct runtime package installs; pre-configure env
```

### 9.3 Scalability Requirements
```
Scalability Requirements:
- Support 100k users in Year 1, scaling to 1M by Year 3 via sharding
- Handle 1M user records with <200ms queries
- Process 50k auth requests/hour; cache sessions in Redis
- Storage growth: 500GB/year for logs/settings
- Horizontal scaling with zero-downtime deploys
- Multi-region support (e.g., US/EU) for low-latency
```

---

## 10. Success Criteria

### 10.1 Key Performance Indicators (KPIs)
```
Business KPIs:
- Reduce admin overhead by 40% (tracked via audit logs) within 6 months
- Achieve 100% compliance audit pass rate by Q2 2026
- Increase user onboarding speed by 50% (from manual to automated)

Technical KPIs:
- 99.9% uptime; <200ms auth response for 99% requests
- 95% test coverage; zero high-risk vulnerabilities post-audit
- Handle 10k concurrent sessions without >5% error rate

User Adoption KPIs:
- 90% admin adoption within 3 months
- <10% support tickets for auth issues monthly
- 85% satisfaction in user surveys (NPS >70)
```

### 10.2 Acceptance Criteria
```
Acceptance Criteria:
- All FRs implemented with unit/integration tests passing >95%
- Security pentest passed with no OWASP Top 10 issues
- Performance benchmarks met under load testing
- UAT by stakeholders with 90% approval rate
- Schema migration successful with 99% data integrity
- Production deploy with zero-downtime; full docs (API/BRD) approved
- Post-launch monitoring shows KPIs trending positive
```

---

## 11. Assumptions & Dependencies

### 11.1 Assumptions
```
Assumptions:
- Stakeholders available for bi-weekly reviews and UAT
- Existing infra (e.g., AWS) supports containerized deploys
- Third-party services (e.g., email APIs) remain stable/unthrottled
- Users have HTTPS-enabled access; basic MFA-capable devices
- Data exports from legacy systems are complete/accurate
- Open-source libs (e.g., Argon2) suffice without custom crypto
```

### 11.2 Dependencies
```
Dependencies:
- Security team approval for RBAC schema and hashing
- DevOps for DB setup (PostgreSQL) and CI/CD pipeline
- Legal/compliance for GDPR-aligned log retention policies
- External providers (e.g., Auth0) API keys/tokens
- Email service (SendGrid) for notifications
- Stakeholder sign-off on FRs before dev start
```

---

## 12. Risks & Mitigation

### 12.1 Risk Assessment
```
Risk 1: Security Breach Exposure
Probability: High
Impact: High
Description: Weak hashing or misconfigured RBAC leads to unauthorized access
Mitigation: Argon2 enforcement, regular pentests, least-privilege audits

Risk 2: Integration Delays
Probability: Medium
Impact: Medium
Description: OAuth/email APIs unstable or rate-limited
Mitigation: Fallback mocks, parallel testing, vendor SLAs

Risk 3: Scalability Bottlenecks
Probability: Medium
Impact: High
Description: Joins/queries slow under load for 1M users
Mitigation: Early load testing, indexing/caching, NoSQL hybrid

Risk 4: Compliance Non-Conformance
Probability: Low
Impact: High
Description: Logs/PII handling violates GDPR
Mitigation: Legal reviews pre-deploy, automated compliance checks
```

### 12.2 Contingency Plans
```
Contingency Plan 1: Auth Integration Failure
- Fallback to basic email/password; disable OAuth temporarily
- Manual admin approvals for high-risk actions
- Hotfix deploy within 24 hours via CI/CD

Contingency Plan 2: Migration Errors
- Rollback to legacy system; run parallel for 2 weeks
- Batch imports with manual QA on samples
- Data recovery from backups with 99% restore SLA
```

---

## 13. Timeline & Milestones

### 13.1 Project Phases
```
Phase 1: Discovery & Planning (3 weeks)
- Refine BRD/tech specs; stakeholder workshops
- Schema design and ERD finalization
- Risk assessment; tech stack PoC (e.g., hashing)

Phase 2: Development (8 weeks)
- Backend APIs and DB implementation
- RBAC logic and auth integrations
- Frontend dashboards; unit/integration tests
- Audit/logging features

Phase 3: Testing & Deployment (4 weeks)
- UAT, security/performance tests
- Data migration dry-runs
- Production deploy; training sessions
- Hypercare monitoring

Phase 4: Optimization (Ongoing, post-launch)
- KPI tracking; feedback iterations
- Scale enhancements (e.g., sharding)
- Compliance audits quarterly
```

### 13.2 Key Milestones
```
Milestone 1: Requirements Sign-Off (Week 3)
- BRD/tech docs approved by stakeholders
- Schema DDL generated; initial PoC passed

Milestone 2: Core MVP (Week 7)
- Provisioning/auth APIs live; basic tests 80% coverage
- Internal demo; feedback incorporated

Milestone 3: Full Integration & Test (Week 11)
- All FRs implemented; UAT complete
- Security audit passed; perf benchmarks met

Milestone 4: Go-Live (Week 15)
- Deploy to prod; migration success
- Training delivered; KPIs baseline established
```

---

## 14. Budget & Resources

### 14.1 Resource Requirements
```
Development Team:
- Project Manager (1 FTE, 15 weeks)
- Backend Developer (2 FTE, 10 weeks)
- Frontend Developer (1 FTE, 8 weeks)
- QA/Security Engineer (1 FTE, 6 weeks)
- DevOps Engineer (0.5 FTE, 4 weeks)
- Compliance Specialist (0.25 FTE, 4 weeks)

Infrastructure Resources:
- Dev/test/prod DB instances (PostgreSQL)
- Cloud hosting (AWS EC2/Lambda)
- API monitoring tools (Prometheus)
- Third-party (SendGrid free tier)
- Dev tools (GitHub, Docker)
```

### 14.2 Budget Allocation
```
Personnel Costs: $80,000
- Project Manager: $15,000
- Backend Developers: $30,000
- Frontend Developer: $15,000
- QA/Security Engineer: $12,000
- DevOps/Compliance: $8,000

Infrastructure Costs: $10,000
- Cloud/DB hosting (Year 1): $6,000
- Tools/services: $3,000
- Testing env: $1,000

Contingency (15%): $13,500

Total Budget: $103,500
```

---
## 4.5. API Architecture: Two-Layer Design

### 4.5.1 Overview

The Sapiens User Management System implements a **two-layer API architecture** that combines:

1. **Layer 1: Backbone Generic CRUD** - Automatic RESTful endpoints for all entities
2. **Layer 2: Domain-Specific Use Cases** - Custom business logic and complex workflows

This architecture provides both **flexibility** (generic CRUD for simple operations) and **power** (custom endpoints for complex business rules).

```
┌─────────────────────────────────────────────────────────────────┐
│                    SAPIENS API ARCHITECTURE                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  LAYER 1: BACKBONE GENERIC CRUD (Auto-generated)                │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  • GET    /api/v1/{collection}                            │  │
│  │  • POST   /api/v1/{collection}                            │  │
│  │  • GET    /api/v1/{collection}/:id                        │  │
│  │  • PUT    /api/v1/{collection}/:id                        │  │
│  │  • PATCH  /api/v1/{collection}/:id                        │  │
│  │  • DELETE /api/v1/{collection}/:id (soft delete)          │  │
│  │  • POST   /api/v1/{collection}/bulk                       │  │
│  │  • POST   /api/v1/{collection}/upcreate                   │  │
│  │  • GET    /api/v1/{collection}/trash                      │  │
│  │  • POST   /api/v1/{collection}/:id/restore                │  │
│  │  • DELETE /api/v1/{collection}/empty                      │  │
│  │                                                            │  │
│  │  Applies to all 12 entities = 132 auto-generated endpoints│  │
│  └───────────────────────────────────────────────────────────┘  │
│                               ▲                                 │
│                               │                                 │
│                    Backbone Framework                           │
│                               │                                 │
│                               ▼                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  LAYER 2: DOMAIN-SPECIFIC USE CASES (Custom business logic)    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  • POST   /api/v1/auth/register           (FR-001)        │  │
│  │  • POST   /api/v1/auth/login              (FR-002)        │  │
│  │  • POST   /api/v1/auth/logout             (FR-002)        │  │
│  │  • POST   /api/v1/auth/forgot-password    (FR-003)        │  │
│  │  • POST   /api/v1/auth/reset-password     (FR-003)        │  │
│  │  • POST   /api/v1/auth/mfa/setup          (FR-004)        │  │
│  │  • POST   /api/v1/auth/mfa/verify         (FR-004)        │  │
│  │  • POST   /api/v1/users/:id/roles         (FR-006)        │  │
│  │  • POST   /api/v1/users/:id/permissions   (FR-006A)       │  │
│  │  • GET    /api/v1/users/:id/effective-permissions         │  │
│  │  • ... (Complex workflows with business rules)            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

### 4.5.2 When to Use Each Layer

#### **Use Layer 1 (Backbone Generic CRUD) when:**
- ✅ Simple CRUD operations without complex business rules
- ✅ Standard pagination, filtering, sorting
- ✅ Bulk operations on entities
- ✅ Trash management (soft delete/restore)
- ✅ Direct database operations with minimal validation

**Examples:**
- `GET /api/v1/roles` - List all roles (simple read)
- `POST /api/v1/permissions` - Create a new permission (simple write)
- `PATCH /api/v1/user_settings/:id` - Update user settings (simple partial update)

#### **Use Layer 2 (Domain-Specific) when:**
- ✅ Complex business logic (password hashing, token generation)
- ✅ Multi-step workflows (user registration with email verification)
- ✅ Cross-entity operations (assign roles, calculate effective permissions)
- ✅ External integrations (send emails, SMS)
- ✅ Security-critical operations (authentication, authorization)

**Examples:**
- `POST /api/v1/auth/register` - Complex: password hashing, email verification, default role assignment
- `POST /api/v1/auth/login` - Complex: credential validation, JWT generation, session creation, MFA
- `POST /api/v1/users/:id/roles` - Complex: role assignment validation, permission inheritance, audit logging

---

### 4.5.3 Layer 1: Backbone Generic CRUD Framework

**Backbone** is a generic CRUD framework that automatically generates RESTful endpoints for all MongoDB collections.

#### **Supported Entities (12 Collections)**

All entities in the system automatically get the 11 standard CRUD endpoints:

1. **users**
2. **roles**
3. **permissions**
4. **user_permissions**
5. **user_roles**
6. **role_permissions**
7. **user_settings**
8. **audit_logs**
9. **sessions**
10. **password_reset_tokens**
11. **mfa_devices**
12. **system_settings**

---

#### **Standard Endpoint Patterns (11 per entity)**

| # | Method | Endpoint Pattern | Description | Soft Delete |
|---|--------|-----------------|-------------|-------------|
| 1 | GET | `/api/v1/{collection}` | List all (paginated, filtered, sorted) | Excludes soft-deleted |
| 2 | POST | `/api/v1/{collection}` | Create new entity | - |
| 3 | GET | `/api/v1/{collection}/:id` | Get single entity by ID | Returns 404 if soft-deleted |
| 4 | PUT | `/api/v1/{collection}/:id` | Full update (replace all fields) | - |
| 5 | PATCH | `/api/v1/{collection}/:id` | Partial update (update specific fields) | - |
| 6 | DELETE | `/api/v1/{collection}/:id` | Soft delete (set `deleted_at`) | Sets `deleted_at` timestamp |
| 7 | POST | `/api/v1/{collection}/bulk` | Bulk create multiple entities | - |
| 8 | POST | `/api/v1/{collection}/upcreate` | Upsert (update if exists, create if not) | - |
| 9 | GET | `/api/v1/{collection}/trash` | List soft-deleted items | Only shows `deleted_at != null` |
| 10 | POST | `/api/v1/{collection}/:id/restore` | Restore soft-deleted item | Sets `deleted_at = null` |
| 11 | DELETE | `/api/v1/{collection}/empty` | Empty trash (hard delete all soft-deleted) | Permanent deletion |

---

#### **Generic Endpoint Examples**

##### **1. List All (GET /{collection})**

**Request:**
```http
GET /api/v1/users?page=1&limit=20&sort=-created_at&status=active
Authorization: Bearer <jwt-token>
```

**Query Parameters:**
| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `page` | Integer | Page number (1-indexed) | `page=1` |
| `limit` | Integer | Items per page (max 100) | `limit=20` |
| `sort` | String | Sort field (prefix `-` for desc) | `sort=-created_at` |
| `{field}` | Any | Filter by field value | `status=active` |

**Response:**
```json
{
  "data": [
    {
      "_id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "john.doe",
      "email": "john.doe@example.com",
      "status": "active",
      "created_at": "2025-01-15T10:30:00Z",
      "updated_at": "2025-01-19T14:20:00Z",
      "deleted_at": null
    }
  ],
  "pagination": {
    "total": 1543,
    "page": 1,
    "limit": 20,
    "total_pages": 78
  }
}
```

---

##### **2. Create New Entity (POST /{collection})**

**Request:**
```http
POST /api/v1/roles
Authorization: Bearer <jwt-token>
Content-Type: application/json

{
  "name": "editor",
  "description": "Content editor role with write permissions",
  "is_default": false
}
```

**Response:**
```json
{
  "_id": "role-550e8400-e29b-41d4-a716-446655440001",
  "name": "editor",
  "description": "Content editor role with write permissions",
  "is_default": false,
  "created_at": "2025-01-19T15:30:00Z",
  "updated_at": "2025-01-19T15:30:00Z",
  "deleted_at": null
}
```

---

##### **3. Get by ID (GET /{collection}/:id)**

**Request:**
```http
GET /api/v1/roles/role-550e8400-e29b-41d4-a716-446655440001
Authorization: Bearer <jwt-token>
```

**Response:**
```json
{
  "_id": "role-550e8400-e29b-41d4-a716-446655440001",
  "name": "editor",
  "description": "Content editor role with write permissions",
  "is_default": false,
  "created_at": "2025-01-19T15:30:00Z",
  "updated_at": "2025-01-19T15:30:00Z",
  "deleted_at": null
}
```

**Error Response (Not Found):**
```json
{
  "error": "NOT_FOUND",
  "message": "Role with ID 'role-123' not found or has been deleted",
  "status": 404
}
```

---

##### **4. Full Update (PUT /{collection}/:id)**

**Request:**
```http
PUT /api/v1/roles/role-550e8400-e29b-41d4-a716-446655440001
Authorization: Bearer <jwt-token>
Content-Type: application/json

{
  "name": "editor",
  "description": "Updated: Content editor with advanced permissions",
  "is_default": false
}
```

**Response:**
```json
{
  "_id": "role-550e8400-e29b-41d4-a716-446655440001",
  "name": "editor",
  "description": "Updated: Content editor with advanced permissions",
  "is_default": false,
  "created_at": "2025-01-19T15:30:00Z",
  "updated_at": "2025-01-19T16:45:00Z",
  "deleted_at": null
}
```

---

##### **5. Partial Update (PATCH /{collection}/:id)**

**Request:**
```http
PATCH /api/v1/user_settings/550e8400-e29b-41d4-a716-446655440000
Authorization: Bearer <jwt-token>
Content-Type: application/json

{
  "theme": "dark",
  "notifications_enabled": false
}
```

**Response:**
```json
{
  "_id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "theme": "dark",
  "language": "en",
  "timezone": "UTC",
  "notifications_enabled": false,
  "updated_at": "2025-01-19T17:00:00Z"
}
```

---

##### **6. Soft Delete (DELETE /{collection}/:id)**

**Request:**
```http
DELETE /api/v1/roles/role-550e8400-e29b-41d4-a716-446655440001
Authorization: Bearer <jwt-token>
```

**Response:**
```json
{
  "message": "Role soft-deleted successfully",
  "id": "role-550e8400-e29b-41d4-a716-446655440001",
  "deleted_at": "2025-01-19T17:15:00Z"
}
```

**Note:** Soft delete sets `deleted_at` timestamp. Entity is hidden from normal queries but can be restored.

---

##### **7. Bulk Create (POST /{collection}/bulk)**

**Request:**
```http
POST /api/v1/permissions/bulk
Authorization: Bearer <jwt-token>
Content-Type: application/json

{
  "items": [
    {
      "name": "read:posts",
      "resource": "posts",
      "description": "Read blog posts"
    },
    {
      "name": "write:posts",
      "resource": "posts",
      "description": "Create and edit blog posts"
    },
    {
      "name": "delete:posts",
      "resource": "posts",
      "description": "Delete blog posts"
    }
  ]
}
```

**Response:**
```json
{
  "created": 3,
  "failed": 0,
  "items": [
    {
      "_id": "perm-001",
      "name": "read:posts",
      "resource": "posts",
      "description": "Read blog posts",
      "created_at": "2025-01-19T17:30:00Z"
    },
    {
      "_id": "perm-002",
      "name": "write:posts",
      "resource": "posts",
      "description": "Create and edit blog posts",
      "created_at": "2025-01-19T17:30:00Z"
    },
    {
      "_id": "perm-003",
      "name": "delete:posts",
      "resource": "posts",
      "description": "Delete blog posts",
      "created_at": "2025-01-19T17:30:00Z"
    }
  ]
}
```

---

##### **8. Upsert (POST /{collection}/upcreate)**

**Request:**
```http
POST /api/v1/user_settings/upcreate
Authorization: Bearer <jwt-token>
Content-Type: application/json

{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "theme": "dark",
  "language": "en",
  "timezone": "America/New_York"
}
```

**Response (Created):**
```json
{
  "operation": "created",
  "_id": "settings-001",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "theme": "dark",
  "language": "en",
  "timezone": "America/New_York",
  "created_at": "2025-01-19T17:45:00Z",
  "updated_at": "2025-01-19T17:45:00Z"
}
```

**Response (Updated):**
```json
{
  "operation": "updated",
  "_id": "settings-001",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "theme": "dark",
  "language": "en",
  "timezone": "America/New_York",
  "created_at": "2025-01-15T10:00:00Z",
  "updated_at": "2025-01-19T17:45:00Z"
}
```

---

##### **9. List Trash (GET /{collection}/trash)**

**Request:**
```http
GET /api/v1/roles/trash?page=1&limit=20
Authorization: Bearer <jwt-token>
```

**Response:**
```json
{
  "data": [
    {
      "_id": "role-550e8400-e29b-41d4-a716-446655440001",
      "name": "editor",
      "description": "Content editor role",
      "created_at": "2025-01-19T15:30:00Z",
      "deleted_at": "2025-01-19T17:15:00Z"
    }
  ],
  "pagination": {
    "total": 12,
    "page": 1,
    "limit": 20,
    "total_pages": 1
  }
}
```

---

##### **10. Restore Deleted Item (POST /{collection}/:id/restore)**

**Request:**
```http
POST /api/v1/roles/role-550e8400-e29b-41d4-a716-446655440001/restore
Authorization: Bearer <jwt-token>
```

**Response:**
```json
{
  "message": "Role restored successfully",
  "_id": "role-550e8400-e29b-41d4-a716-446655440001",
  "name": "editor",
  "deleted_at": null,
  "restored_at": "2025-01-19T18:00:00Z"
}
```

---

##### **11. Empty Trash (DELETE /{collection}/empty)**

**Request:**
```http
DELETE /api/v1/roles/empty
Authorization: Bearer <jwt-token>
```

**Response:**
```json
{
  "message": "Trash emptied successfully",
  "deleted_count": 12,
  "deleted_permanently": true
}
```

**Warning:** This is a **permanent deletion** (hard delete). Cannot be undone.

---

### 4.5.4 Backbone Endpoints by Entity

#### **Complete Endpoint Matrix (132 Total Endpoints)**

| Entity | Base Path | GET List | POST Create | GET :id | PUT :id | PATCH :id | DELETE :id | POST bulk | POST upcreate | GET trash | POST restore | DELETE empty |
|--------|-----------|----------|-------------|---------|---------|-----------|------------|-----------|---------------|-----------|--------------|--------------|
| **users** | `/api/v1/users` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **roles** | `/api/v1/roles` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **permissions** | `/api/v1/permissions` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **user_permissions** | `/api/v1/user_permissions` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **user_roles** | `/api/v1/user_roles` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **role_permissions** | `/api/v1/role_permissions` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **user_settings** | `/api/v1/user_settings` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **audit_logs** | `/api/v1/audit_logs` | ✅ | ⚠️ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ |
| **sessions** | `/api/v1/sessions` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **password_reset_tokens** | `/api/v1/password_reset_tokens` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **mfa_devices** | `/api/v1/mfa_devices` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **system_settings** | `/api/v1/system_settings` | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

**Legend:**
- ✅ = Fully supported
- ⚠️ = Supported with restrictions (audit_logs: create only via system, no manual creation recommended)
- ❌ = Disabled for security/compliance (audit_logs are immutable)

**Note on audit_logs:**
- **Read-only** via Backbone (GET list, GET :id, GET trash)
- **Create** should only happen via system audit logger, not manual POST
- **Update/Delete** disabled to maintain audit trail integrity
- Use domain-specific endpoints for audit log queries

---

### 4.5.5 Backbone Generic Features

#### **Pagination**
All `GET /{collection}` endpoints support pagination:
- `page` - Page number (1-indexed, default: 1)
- `limit` - Items per page (default: 20, max: 100)

#### **Filtering**
Filter by any field using query parameters:
- `GET /api/v1/users?status=active&role=admin`
- `GET /api/v1/sessions?user_id=550e8400-e29b-41d4-a716-446655440000`

#### **Sorting**
Sort by any field (prefix `-` for descending):
- `GET /api/v1/users?sort=-created_at` (newest first)
- `GET /api/v1/users?sort=username` (alphabetical)

#### **Soft Delete**
All entities support soft delete:
- `DELETE /{collection}/:id` sets `deleted_at` timestamp
- Soft-deleted items excluded from normal queries
- Use `/trash` endpoint to view deleted items
- Use `/restore` to recover deleted items
- Use `/empty` to permanently delete (hard delete)

#### **Validation**
Backbone validates:
- Required fields (based on MongoDB schema)
- Field types (string, number, boolean, date)
- Field constraints (min/max length, enum values)
- Unique constraints (email, username)

#### **Error Handling**
Standard HTTP status codes:
- `200 OK` - Success
- `201 Created` - Entity created
- `400 Bad Request` - Validation error
- `401 Unauthorized` - Missing/invalid JWT
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Entity not found or soft-deleted
- `409 Conflict` - Duplicate unique field
- `500 Internal Server Error` - Server error

---

### 4.5.6 Security & Permissions

#### **Authentication**
All Backbone endpoints require JWT authentication:
```http
Authorization: Bearer <jwt-token>
```

#### **Permission Patterns**
Generic CRUD operations use standard permission patterns:

| Operation | Required Permission Pattern |
|-----------|---------------------------|
| GET /{collection} | `read:{collection}` |
| POST /{collection} | `create:{collection}` or `write:{collection}` |
| GET /{collection}/:id | `read:{collection}` |
| PUT /{collection}/:id | `update:{collection}` or `write:{collection}` |
| PATCH /{collection}/:id | `update:{collection}` or `write:{collection}` |
| DELETE /{collection}/:id | `delete:{collection}` |
| POST /{collection}/bulk | `create:{collection}` or `write:{collection}` |
| POST /{collection}/upcreate | `create:{collection}` or `write:{collection}` |
| GET /{collection}/trash | `read:{collection}` or `admin:{collection}` |
| POST /{collection}/:id/restore | `delete:{collection}` or `admin:{collection}` |
| DELETE /{collection}/empty | `admin:{collection}` |

**Examples:**
- Read users: Requires `read:users` permission
- Create role: Requires `create:roles` or `write:roles` permission
- Delete permission: Requires `delete:permissions` permission
- Empty audit logs trash: Requires `admin:audit_logs` permission

---

### 4.5.7 Rate Limiting

Backbone endpoints are rate-limited to prevent abuse:

| Endpoint Type | Rate Limit | Window |
|--------------|------------|--------|
| GET (list) | 100 requests | per minute |
| GET (by ID) | 200 requests | per minute |
| POST (create) | 50 requests | per minute |
| PUT/PATCH (update) | 50 requests | per minute |
| DELETE (soft delete) | 30 requests | per minute |
| POST (bulk) | 10 requests | per minute |
| DELETE (empty trash) | 5 requests | per hour |

**Rate limit headers:**
```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1705680000
```

---

## 5. Functional Requirements (Detailed)

**Note:** The following functional requirements describe **Layer 2: Domain-Specific Use Cases** that implement custom business logic beyond generic CRUD operations provided by Backbone.

### 5.1 User Provisioning Requirements

#### FR-001: Create User Account (Priority: HIGH)

**Description**: Allow administrators or self-registration to create new user records with email verification and default role assignment.

**Actors**: System Administrator, End User (self-registration), Automated System

**Preconditions**:
- For admin creation: Admin is authenticated with `create:user` permission
- For self-registration: Registration is enabled in system settings
- Email service is available for verification

**Inputs**:
| Field | Type | Constraints | Required |
|-------|------|-------------|----------|
| email | String | Valid RFC 5322 email, lowercase, unique | Yes |
| username | String | 3-50 chars, alphanumeric + _/-, unique | Yes |
| password | String | Min 12 chars, complexity requirements | Yes |
| first_name | String | Max 100 chars, letters + spaces/apostrophes/hyphens | Yes |
| last_name | String | Max 100 chars, letters + spaces/apostrophes/hyphens | Yes |
| phone_number | String | E.164 format (optional) | No |
| profile_picture_url | String | Valid URL, max 500 chars (optional) | No |

**Processing Steps**:
1. **Validation**:
   - Validate email format (RFC 5322 regex)
   - Check email uniqueness in database
   - Validate username format and uniqueness
   - Validate password strength (min 12 chars, uppercase, lowercase, digit, special char)
   - Validate name formats
   - Validate phone number format (E.164) if provided

2. **Password Hashing**:
   - Hash password using Argon2id algorithm
   - Parameters: `m=19456, t=2, p=1, output=32 bytes`
   - Store hash (min 60 chars) in `password_hash` field

3. **Role Assignment**:
   - Auto-assign default 'user' role via UserRoles junction
   - Insert record into `user_roles` table with current timestamp
   - For admin-created users: optionally assign custom roles

4. **User Creation**:
   - Generate UUIDv4 for user ID
   - Set status to 'active'
   - Set `created_at` and `updated_at` to current UTC timestamp
   - Insert user record into `users` collection/table

5. **Verification Email**:
   - Generate unique verification token (JWT, 24-hour expiry)
   - Send email with verification link to user's email
   - Log email send event in audit logs

**Outputs**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john.doe",
  "email": "john.doe@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "status": "active",
  "created_at": "2025-11-19T10:30:00Z",
  "roles": ["user"]
}
```

**Acceptance Criteria**:
- ✅ Email uniqueness is validated before creation
- ✅ Password is hashed with Argon2id before storage
- ✅ Default 'user' role is auto-assigned
- ✅ Confirmation email is sent with verification link
- ✅ User record is created with all required fields
- ✅ Audit log entry is created for user creation event
- ✅ API returns 201 Created with user object (excluding password_hash)
- ✅ Duplicate email returns 409 Conflict error
- ✅ Invalid password returns 400 Bad Request with details

**Error Handling**:
| Error Condition | HTTP Status | Error Code | Message |
|----------------|-------------|------------|---------|
| Duplicate email | 409 | USER_EMAIL_EXISTS | "Email address already registered" |
| Duplicate username | 409 | USER_USERNAME_EXISTS | "Username already taken" |
| Weak password | 400 | PASSWORD_WEAK | "Password does not meet complexity requirements" |
| Invalid email | 400 | EMAIL_INVALID | "Invalid email format" |
| Email service unavailable | 500 | EMAIL_SERVICE_ERROR | "Failed to send verification email" |

**Performance Requirements**:
- User creation shall complete in <500ms (excluding email send)
- Uniqueness check shall use indexed fields (email, username)
- Bulk user import shall support 100+ users/minute

**Security Requirements**:
- Password must never be stored in plain text
- Password must never be returned in API responses
- Verification token must be cryptographically secure
- Rate limit: 5 account creations per IP per hour (prevent spam)

**Audit Requirements**:
- Log user creation event with:
  - User ID
  - Creator ID (admin or 'self-registration')
  - Timestamp
  - IP address
  - User agent

---

#### FR-002: Update User Profile (Priority: HIGH)

**Description**: Allow users/admins to modify user profile fields with verification for sensitive changes.

**Actors**: System Administrator, End User (self), Automated System

**Preconditions**:
- User is authenticated
- For admin updates: Admin has `update:user` permission
- For self-updates: User can only update non-sensitive fields

**Inputs**:
| Field | Updateable by User | Updateable by Admin | Verification Required |
|-------|-------------------|---------------------|----------------------|
| first_name | Yes | Yes | No |
| last_name | Yes | Yes | No |
| phone_number | Yes | Yes | No |
| profile_picture_url | Yes | Yes | No |
| email | No | Yes | Yes (email verification) |
| username | No | Yes | No |
| status | No | Yes | No |
| roles | No | Yes | No (separate FR-006) |

**Processing Steps**:
1. **Authorization Check**:
   - If self-update: Verify user is updating own profile
   - If admin-update: Verify admin has `update:user` permission
   - Deny updates to sensitive fields by non-admins

2. **Validation**:
   - Validate updated field formats
   - For email changes: Check uniqueness
   - For phone number: Validate E.164 format

3. **Email Change Flow** (if email updated):
   - Send verification email to NEW email address
   - Do not update email until verified
   - Create pending email change record
   - Set expiry (24 hours)

4. **Update Execution**:
   - Update specified fields in users collection
   - Update `updated_at` timestamp
   - Maintain version for optimistic locking

5. **Audit Logging**:
   - Log which fields were changed
   - Store old and new values (except password)
   - Record updater ID and timestamp

**Outputs**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "updated_fields": ["first_name", "phone_number"],
  "updated_at": "2025-11-19T11:45:00Z"
}
```

**Acceptance Criteria**:
- ✅ Users can update own non-sensitive fields
- ✅ Admins can update all fields
- ✅ Email changes require verification
- ✅ `updated_at` timestamp is automatically updated
- ✅ Optimistic locking prevents concurrent update conflicts
- ✅ Audit log records old and new values
- ✅ Non-admins cannot change email, username, status, roles

**Error Handling**:
| Error Condition | HTTP Status | Error Code | Message |
|----------------|-------------|------------|---------|
| Unauthorized update | 403 | FORBIDDEN | "Insufficient permissions to update field" |
| Duplicate email | 409 | USER_EMAIL_EXISTS | "Email address already in use" |
| Invalid phone format | 400 | PHONE_INVALID | "Invalid phone number format (use E.164)" |
| Concurrent update | 409 | VERSION_CONFLICT | "Record was modified by another user" |

---

#### FR-003: Delete/Suspend User Account (Priority: HIGH)

**Description**: Soft-delete or suspend user accounts with cascading updates and data retention.

**Actors**: System Administrator

**Preconditions**:
- Admin is authenticated with `delete:user` or `suspend:user` permission
- User account exists and is not already deleted

**Operations**:

**1. Soft Delete**:
- Set `status` to 'inactive'
- Set `deleted_at` timestamp
- Retain all user data for 90 days (GDPR compliance)
- Remove from active user queries
- Cascade: Remove UserRoles assignments
- Keep: Audit logs for compliance

**2. Suspend**:
- Set `status` to 'suspended'
- Add `suspension_reason` and `suspended_until` metadata
- Prevent login attempts
- Maintain all roles and permissions
- Reversible by admin

**Processing Steps**:
1. **Validation**:
   - Verify user exists
   - Check admin has appropriate permission
   - Prevent self-deletion for admins

2. **Status Update**:
   - Update `status` field ('inactive' or 'suspended')
   - Set `deleted_at` (for soft delete) or `suspended_until`
   - Add `reason` to metadata

3. **Cascade Operations**:
   - For soft delete: Remove UserRoles entries
   - For suspension: Keep roles intact
   - Revoke all active JWT tokens (add to blacklist)
   - Mark active sessions as invalid

4. **Notification**:
   - Send notification email to user (configurable)
   - Notify system administrators

5. **Audit Logging**:
   - Log deletion/suspension event
   - Record reason and admin who performed action

**Outputs**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "suspended",
  "suspended_until": "2025-12-19T00:00:00Z",
  "reason": "Policy violation: spam activity"
}
```

**Acceptance Criteria**:
- ✅ Soft delete sets status to 'inactive' and `deleted_at` timestamp
- ✅ Data is retained for 90 days before permanent deletion
- ✅ Suspended users cannot login
- ✅ UserRoles removed on soft delete, kept on suspension
- ✅ Active sessions are invalidated
- ✅ Notification email sent to user
- ✅ Audit log created with reason
- ✅ Admins cannot delete themselves

**Data Retention**:
- Soft-deleted users: 90 days retention
- After 90 days: Permanent deletion with anonymization
- Audit logs: Retained for 1 year (cannot be deleted)
- User-generated content: Handle per application policy

---

### 5.2 Authentication Requirements

#### FR-004: Login Verification (Priority: HIGH)

**Description**: Support email/password, MFA, and OAuth social logins with account lockout protection.

**Actors**: End User, External OAuth Provider

**Preconditions**:
- User account exists and is active
- For MFA: User has enrolled MFA device
- For OAuth: OAuth provider is configured

**Authentication Methods**:

**1. Email/Password Authentication**:

**Inputs**:
```json
{
  "email_or_username": "john.doe@example.com",
  "password": "SecureP@ssw0rd123"
}
```

**Processing Steps**:
1. **User Lookup**:
   - Search by email OR username (flexible)
   - Check user exists and status is 'active'
   - Verify account is not locked

2. **Password Verification**:
   - Retrieve `password_hash` from database
   - Verify password using Argon2id.verify()
   - Constant-time comparison to prevent timing attacks

3. **Failed Attempt Handling**:
   - Increment `failed_login_attempts` counter
   - If attempts >= 5: Lock account for 5 minutes
   - Log failed attempt with IP and user agent

4. **Success Path**:
   - Reset `failed_login_attempts` to 0
   - Update `last_login` timestamp
   - Check if MFA is required
   - Generate JWT token (if no MFA) or MFA challenge token

5. **MFA Challenge** (if enabled):
   - Generate temporary MFA challenge token (5 min expiry)
   - Return MFA challenge response
   - Require MFA code verification before JWT issuance

**Outputs (Success - No MFA)**:
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john.doe",
    "email": "john.doe@example.com",
    "roles": ["user", "editor"]
  }
}
```

**Outputs (MFA Required)**:
```json
{
  "mfa_required": true,
  "mfa_challenge_token": "temp_mfa_abc123",
  "mfa_methods": ["totp", "sms"]
}
```

**2. Multi-Factor Authentication (MFA)**:

**TOTP (Time-based One-Time Password)**:
- Algorithm: HMAC-SHA1
- Period: 30 seconds
- Digits: 6
- Support Google Authenticator, Authy, Microsoft Authenticator

**SMS-based MFA**:
- Send 6-digit code to registered phone number
- Code expires in 5 minutes
- Rate limit: 3 codes per 15 minutes

**Processing Steps**:
1. Receive MFA code from user
2. Validate code against stored secret (TOTP) or sent code (SMS)
3. Allow time window tolerance: ±1 period (30 sec) for TOTP
4. On success: Issue JWT token
5. On failure: Increment MFA attempt counter, lock after 3 failures

**3. OAuth 2.0 Social Login**:

**Supported Providers**:
- Google OAuth 2.0
- GitHub OAuth
- Microsoft Azure AD

**Processing Steps**:
1. Redirect user to OAuth provider authorization URL
2. Receive authorization code callback
3. Exchange code for access token
4. Fetch user profile from provider
5. Match or create user:
   - If email exists: Link OAuth provider to existing account
   - If email new: Create new user with OAuth profile data
6. Generate JWT token for user

**Acceptance Criteria**:
- ✅ Support email OR username for login
- ✅ Password verification uses constant-time comparison
- ✅ Account locked after 5 failed attempts (15-min lockout)
- ✅ MFA enrollment during first login for admins
- ✅ JWT token contains user ID, roles, expiry
- ✅ Refresh token allows extended sessions
- ✅ OAuth providers: Google, GitHub, Microsoft
- ✅ OAuth user profile synced on first login
- ✅ All login attempts logged to audit logs

**Error Handling**:
| Error Condition | HTTP Status | Error Code | Message |
|----------------|-------------|------------|---------|
| Invalid credentials | 401 | AUTH_INVALID_CREDENTIALS | "Invalid email or password" |
| Account locked | 423 | AUTH_ACCOUNT_LOCKED | "Account locked due to failed attempts. Try again in 5 minutes." |
| Account suspended | 403 | AUTH_ACCOUNT_SUSPENDED | "Account has been suspended" |
| MFA code invalid | 401 | AUTH_MFA_INVALID | "Invalid MFA code" |
| MFA code expired | 401 | AUTH_MFA_EXPIRED | "MFA code has expired. Request a new code." |

**Security Requirements**:
- Rate limiting: 10 login attempts per IP per minute
- Account lockout: 5 failed attempts = 5 minute lockout
- JWT expiry: 24 hours for access token, 7 days for refresh token
- Secure token storage: HttpOnly, Secure, SameSite cookies
- CSRF protection for OAuth callbacks

---

#### FR-005: Session Management (Priority: MEDIUM)

**Description**: Generate, validate, refresh, and revoke JWT-based session tokens.

**JWT Token Structure**:
```json
{
  "header": {
    "alg": "HS256",
    "typ": "JWT"
  },
  "payload": {
    "sub": "550e8400-e29b-41d4-a716-446655440000",
    "email": "john.doe@example.com",
    "roles": ["user", "editor"],
    "permissions": ["read:users", "write:posts"],
    "iat": 1700000000,
    "exp": 1700086400,
    "jti": "unique-token-id"
  }
}
```

**Operations**:

**1. Token Generation**:
- Generate on successful login
- Include user ID, email, roles, permissions
- Set expiry (24 hours for access, 7 days for refresh)
- Generate unique JWT ID (jti) for revocation tracking
- Sign with secret key (HS256 or RS256)

**2. Token Validation**:
- Verify signature
- Check expiry timestamp
- Validate token is not blacklisted (revoked)
- Extract user claims (ID, roles, permissions)

**3. Token Refresh**:
- Accept refresh token
- Validate refresh token not expired/revoked
- Issue new access token
- Optionally issue new refresh token (rotate)

**4. Token Revocation**:
- Add JWT ID (jti) to blacklist (Redis)
- Set TTL matching token expiry
- Invalidate all user sessions on logout/password change

**Acceptance Criteria**:
- ✅ Access tokens expire in 24 hours
- ✅ Refresh tokens expire in 7 days
- ✅ Blacklisted tokens are rejected
- ✅ Token refresh rotates refresh token (optional)
- ✅ User permissions included in token payload
- ✅ Logout invalidates all active sessions
- ✅ Password change invalidates all sessions

**Performance Requirements**:
- Token validation: <10ms
- Blacklist check (Redis): <5ms
- Token generation: <50ms

---

### 5.3 Authorization Requirements

#### FR-006: Role and Permission Assignment (Priority: HIGH)

**Description**: Admins assign roles to users; roles inherit permissions through junction tables.

**Actors**: System Administrator

**Preconditions**:
- Admin has `assign_role` permission
- User exists
- Role exists

**Operations**:

**1. Assign Role to User**:

**Input**:
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "role_id": "admin"
}
```

**Processing**:
1. Verify admin has `assign_role` permission
2. Check user and role exist
3. Check role not already assigned
4. Insert into `user_roles` junction table
5. Update `assigned_at` timestamp
6. Log role assignment in audit logs

**2. Remove Role from User**:
- Delete from `user_roles` junction
- Log removal in audit logs

**3. Get User's Effective Permissions**:

**Query Flow**:
```sql
SELECT DISTINCT p.name, p.resource, p.description
FROM users u
JOIN user_roles ur ON u.id = ur.user_id
JOIN roles r ON ur.role_id = r.id
JOIN role_permissions rp ON r.id = rp.role_id
JOIN permissions p ON rp.permission_id = p.id
WHERE u.id = ?
```

**Outputs**:
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "roles": ["user", "editor"],
  "permissions": [
    {
      "name": "read:users",
      "resource": "users",
      "description": "Read user profiles"
    },
    {
      "name": "write:posts",
      "resource": "posts",
      "description": "Create and edit posts"
    }
  ]
}
```

**Acceptance Criteria**:
- ✅ Many-to-many relationship via UserRoles junction
- ✅ Cascade permission checks for effective perms
- ✅ All assignments logged with timestamp and IP
- ✅ Deny if user lacks `assign_role` permission
- ✅ Duplicate role assignment returns 409 Conflict

---

#### FR-006A: Direct User Permission Assignment (Priority: MEDIUM)

**Description**: Admins grant permissions directly to individual users for exception cases, temporary access, or resource-specific permissions that bypass standard role-based assignments. This implements the hybrid RBAC model where effective permissions = role-based permissions ∪ direct user permissions.

**Actors**: System Administrator, Security Officer

**Preconditions**:
- Admin has `assign_permission` permission
- User exists and is active
- Permission exists
- Business justification provided for direct permission grant

**Operations**:

**1. Grant Direct Permission to User**:

**Input**:
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "permission_id": "delete:sensitive_records",
  "expires_at": "2025-12-31T23:59:59Z",  // Optional: temporary access
  "reason": "Temporary access for Q4 2025 audit cleanup project",
  "resource_type": "audit_records",  // Optional: scope to specific resource type
  "resource_id": "audit-2025-q4",  // Optional: scope to specific resource instance
  "granted_by": "admin-550e8400-e29b-41d4-a716-446655440001"
}
```

**Processing**:
1. **Validation**:
   - Verify admin has `assign_permission` permission
   - Validate `user_id`, `permission_id` exist
   - Validate `expires_at` is in future (if provided)
   - Validate `reason` is 10-500 characters
   - Validate resource scoping is valid (if provided)

2. **Existence Checks**:
   - Query `users` collection for user existence and active status
   - Query `permissions` collection for permission existence
   - If resource scoping specified, verify resource exists

3. **Duplicate Check**:
   - Query `user_permissions` for existing active grant:
     ```javascript
     db.user_permissions.findOne({
       user_id: userId,
       permission_id: permissionId,
       is_active: true,
       $or: [
         { expires_at: null },
         { expires_at: { $gte: new Date() } }
       ]
     })
     ```
   - If duplicate found, return 409 Conflict error

4. **Insert Direct Permission**:
   ```javascript
   db.user_permissions.insertOne({
     _id: UUID(),
     user_id: userId,
     permission_id: permissionId,
     granted_at: new Date(),
     granted_by: adminId,
     expires_at: expiresAt || null,
     reason: reason,
     resource_id: resourceId || null,
     resource_type: resourceType || null,
     is_active: true,
     revoked_at: null,
     revoked_by: null,
     revoked_reason: null
   })
   ```

5. **Audit Logging**:
   ```javascript
   db.audit_logs.insertOne({
     _id: UUID(),
     user_id: adminId,
     action: "GRANT_DIRECT_PERMISSION",
     resource_type: "user_permissions",
     resource_id: grantId,
     details: {
       target_user_id: userId,
       permission_id: permissionId,
       expires_at: expiresAt,
       reason: reason,
       resource_scoping: { resource_type, resource_id }
     },
     ip_address: requestIp,
     user_agent: requestUserAgent,
     timestamp: new Date()
   })
   ```

**Output**:
```json
{
  "id": "permission-grant-123",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "permission": {
    "id": "delete:sensitive_records",
    "name": "Delete Sensitive Records",
    "resource": "audit_records",
    "description": "Ability to delete sensitive audit records"
  },
  "granted_at": "2025-01-19T10:30:00Z",
  "granted_by": {
    "id": "admin-550e8400-e29b-41d4-a716-446655440001",
    "username": "admin.sarah"
  },
  "expires_at": "2025-12-31T23:59:59Z",
  "reason": "Temporary access for Q4 2025 audit cleanup project",
  "resource_scoping": {
    "resource_type": "audit_records",
    "resource_id": "audit-2025-q4"
  },
  "is_active": true,
  "status": "granted"
}
```

**2. Revoke Direct Permission**:

**Input**:
```json
{
  "grant_id": "permission-grant-123",
  "revoked_reason": "Project completed earlier than expected"
}
```

**Processing**:
1. Verify admin has `revoke_permission` permission
2. Update `user_permissions`:
   ```javascript
   db.user_permissions.updateOne(
     { _id: grantId },
     {
       $set: {
         is_active: false,
         revoked_at: new Date(),
         revoked_by: adminId,
         revoked_reason: revokedReason
       }
     }
   )
   ```
3. Log revocation in audit logs

**3. Get User's Effective Permissions (Hybrid RBAC)**:

**Query Flow**:
```javascript
// 1. Get role-based permissions
const rolePermissions = await db.users.aggregate([
  { $match: { _id: userId } },
  { $lookup: {
      from: "user_roles",
      localField: "_id",
      foreignField: "user_id",
      as: "user_roles"
  }},
  { $unwind: "$user_roles" },
  { $lookup: {
      from: "role_permissions",
      localField: "user_roles.role_id",
      foreignField: "role_id",
      as: "role_perms"
  }},
  { $unwind: "$role_perms" },
  { $lookup: {
      from: "permissions",
      localField: "role_perms.permission_id",
      foreignField: "_id",
      as: "permission"
  }},
  { $unwind: "$permission" },
  { $group: {
      _id: "$permission._id",
      permission: { $first: "$permission" },
      source: { $literal: "role" }
  }}
])

// 2. Get direct user permissions (active and not expired)
const directPermissions = await db.user_permissions.aggregate([
  { $match: {
      user_id: userId,
      is_active: true,
      $or: [
        { expires_at: null },
        { expires_at: { $gte: new Date() } }
      ]
  }},
  { $lookup: {
      from: "permissions",
      localField: "permission_id",
      foreignField: "_id",
      as: "permission"
  }},
  { $unwind: "$permission" },
  { $project: {
      _id: "$permission._id",
      permission: "$permission",
      source: { $literal: "direct" },
      expires_at: 1,
      resource_scoping: {
        resource_type: "$resource_type",
        resource_id: "$resource_id"
      }
  }}
])

// 3. Union: Effective Permissions = Role Permissions ∪ Direct Permissions
const effectivePermissions = [...rolePermissions, ...directPermissions]
  .reduce((acc, curr) => {
    if (!acc.some(p => p._id === curr._id)) {
      acc.push(curr)
    }
    return acc
  }, [])
```

**Output**:
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "effective_permissions": [
    {
      "id": "read:users",
      "name": "Read Users",
      "resource": "users",
      "description": "Read user profiles",
      "source": "role",
      "via_roles": ["user", "editor"]
    },
    {
      "id": "delete:sensitive_records",
      "name": "Delete Sensitive Records",
      "resource": "audit_records",
      "description": "Delete sensitive audit records",
      "source": "direct",
      "expires_at": "2025-12-31T23:59:59Z",
      "resource_scoping": {
        "resource_type": "audit_records",
        "resource_id": "audit-2025-q4"
      },
      "granted_reason": "Temporary access for Q4 2025 audit cleanup project"
    }
  ],
  "permission_summary": {
    "total": 12,
    "from_roles": 11,
    "direct_grants": 1,
    "expiring_soon": 1
  }
}
```

**4. List All Direct Permission Grants (Admin View)**:

**Input**:
```json
{
  "filters": {
    "user_id": "550e8400-e29b-41d4-a716-446655440000",  // Optional
    "permission_id": "delete:sensitive_records",  // Optional
    "is_active": true,  // Optional
    "expiring_within_days": 30  // Optional
  },
  "pagination": {
    "page": 1,
    "page_size": 50
  },
  "sort": {
    "field": "granted_at",
    "order": "desc"
  }
}
```

**Output**:
```json
{
  "grants": [
    {
      "id": "permission-grant-123",
      "user": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "john.doe",
        "email": "john.doe@example.com"
      },
      "permission": {
        "id": "delete:sensitive_records",
        "name": "Delete Sensitive Records"
      },
      "granted_at": "2025-01-19T10:30:00Z",
      "granted_by": {
        "id": "admin-001",
        "username": "admin.sarah"
      },
      "expires_at": "2025-12-31T23:59:59Z",
      "days_until_expiry": 346,
      "reason": "Temporary access for Q4 2025 audit cleanup project",
      "resource_scoping": {
        "resource_type": "audit_records",
        "resource_id": "audit-2025-q4"
      },
      "is_active": true
    }
  ],
  "pagination": {
    "total": 87,
    "page": 1,
    "page_size": 50,
    "total_pages": 2
  }
}
```

**Acceptance Criteria**:
- ✅ Direct permissions can be granted with business justification (reason field required)
- ✅ Permissions can have optional expiry dates for temporary access
- ✅ Permissions can be scoped to specific resource types or instances
- ✅ Effective permissions query returns union of role-based + direct permissions
- ✅ Duplicate active grants for same user-permission pair are prevented (409 Conflict)
- ✅ All grants and revocations logged in audit logs with full context
- ✅ Admins can list/filter all direct permission grants
- ✅ Expired permissions automatically excluded from effective permissions
- ✅ Revoked permissions marked as inactive (soft delete)
- ✅ Permission checks account for resource scoping when specified

**Error Handling**:

| Error Code | Condition | HTTP Status | Response |
|------------|-----------|-------------|----------|
| PERMISSION_DENIED | Admin lacks `assign_permission` | 403 | `{"error": "Insufficient permissions to grant direct permissions"}` |
| USER_NOT_FOUND | Target user doesn't exist | 404 | `{"error": "User not found"}` |
| PERMISSION_NOT_FOUND | Permission ID invalid | 404 | `{"error": "Permission not found"}` |
| DUPLICATE_GRANT | Active grant already exists | 409 | `{"error": "Active permission grant already exists for this user"}` |
| INVALID_EXPIRY | `expires_at` in past | 400 | `{"error": "Expiry date must be in the future"}` |
| INVALID_REASON | Reason <10 or >500 chars | 400 | `{"error": "Reason must be 10-500 characters"}` |
| RESOURCE_NOT_FOUND | Resource scoping references non-existent resource | 404 | `{"error": "Scoped resource not found"}` |
| GRANT_NOT_FOUND | Grant ID doesn't exist | 404 | `{"error": "Permission grant not found"}` |

**Security Requirements**:
- Only users with `assign_permission` can grant direct permissions
- Only users with `revoke_permission` can revoke direct permissions
- All operations require valid JWT authentication
- Rate limiting: 100 grant/revoke operations per admin per hour
- Sensitive permissions (e.g., `delete:users`, `admin:*`) require additional confirmation
- Grant reason is mandatory and immutable (audit trail)

**Performance Requirements**:
- Permission grant operation: <100ms (p95)
- Effective permissions query: <200ms for users with <100 total permissions (p95)
- Direct permission listing: <500ms for result sets <1000 grants (p95)
- Expired permission cleanup job runs daily at 00:00 UTC

**Audit Requirements**:
- Log all direct permission grants with full context (who, what, why, when, expires)
- Log all permission revocations with reason
- Monthly report: All active direct permissions by user
- Alert: Direct permissions expiring within 7 days
- Alert: Direct permissions granted without expiry (require review every 90 days)

**UI/UX Considerations**:
- Admin UI shows visual distinction between role-based and direct permissions
- Warning when granting permission without expiry date
- Auto-suggest expiry dates based on common durations (7 days, 30 days, 90 days, 1 year)
- Bulk revocation for multiple grants (e.g., revoke all grants for a user)
- Export direct permission grants to CSV for compliance audits

**Best Practices & Governance**:
- Direct permissions should be exception-based, not standard practice
- Prefer role-based permissions for permanent access
- Always set expiry dates for temporary access
- Require detailed business justification in reason field
- Regular audits of direct permissions (quarterly reviews)
- Auto-expire direct permissions after 90 days if no expiry set
- Limit direct permission grants to <5% of total user base

---

#### FR-007: Access Control Enforcement (Priority: HIGH)

**Description**: Middleware checks user roles/permissions before resource access.

**Authorization Middleware Flow**:
```
1. Extract JWT from request (Authorization header)
2. Validate JWT signature and expiry
3. Extract user ID and permissions from JWT
4. Check required permission for requested resource/action
5. Allow if permission exists, Deny if not
6. Log denied accesses
```

**Permission Format**:
- Pattern: `<action>:<resource>`
- Examples:
  - `read:users` - Read user profiles
  - `write:users` - Create/update users
  - `delete:users` - Delete users
  - `assign_role` - Assign roles to users

**Hierarchical Roles**:
- `admin` role inherits all `editor` permissions
- `editor` role inherits all `user` permissions
- Hierarchy defined in role configuration

**Acceptance Criteria**:
- ✅ Middleware checks permissions on all protected routes
- ✅ Deny-by-default security model
- ✅ Hierarchical roles supported
- ✅ All denied accesses logged to audit
- ✅ 403 Forbidden returned for insufficient permissions

---

### 5.4 Auditing & Logging Requirements

#### FR-008: Event Tracking (Priority: MEDIUM)

**Description**: Log all user actions in immutable audit logs for compliance and security monitoring.

**Events to Log**:
| Event Type | Trigger | Data Captured |
|------------|---------|---------------|
| user_created | User account created | user_id, creator_id, email, roles |
| user_updated | Profile updated | user_id, fields_changed, old_values, new_values |
| user_deleted | Account deleted | user_id, deletion_type (soft/hard), reason |
| user_suspended | Account suspended | user_id, reason, suspended_until |
| login_success | Successful login | user_id, auth_method (password/oauth/mfa), ip, user_agent |
| login_failure | Failed login | email_or_username, reason, ip, user_agent, attempts |
| logout | User logout | user_id, session_id |
| mfa_enrolled | MFA device enrolled | user_id, mfa_method (totp/sms) |
| mfa_disabled | MFA disabled | user_id, disabled_by |
| role_assigned | Role assigned to user | user_id, role_id, assigned_by |
| role_removed | Role removed from user | user_id, role_id, removed_by |
| permission_denied | Access denied | user_id, resource, action, required_permission |
| password_changed | Password updated | user_id, changed_by (self/admin) |
| password_reset_requested | Reset link sent | user_id, email |
| password_reset_completed | Reset completed | user_id |
| email_changed | Email updated | user_id, old_email, new_email |
| session_revoked | Session invalidated | user_id, session_id, revoked_by, reason |

**Audit Log Structure**:
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "action": "login_success",
  "details": {
    "auth_method": "password",
    "mfa_used": true
  },
  "ip_address": "192.168.1.100",
  "user_agent": "Mozilla/5.0...",
  "timestamp": "2025-11-19T12:00:00Z",
  "session_id": "uuid"
}
```

**Retention Policy**:
- Audit logs: 1 year minimum (GDPR requirement)
- After 1 year: Archive to cold storage (S3 Glacier)
- Never delete audit logs (immutable)

**Query Capabilities**:
- Filter by user_id
- Filter by action type
- Filter by date range
- Filter by IP address
- Full-text search on details

**Acceptance Criteria**:
- ✅ All user actions logged to AuditLogs
- ✅ Logs are immutable (no updates/deletes)
- ✅ Logs retained for 1 year minimum
- ✅ Queryable by user/date/action
- ✅ Sensitive data (passwords) not logged
- ✅ PII anonymized in logs per GDPR

---

#### FR-009: Anomaly Alerts (Priority: LOW)

**Description**: Notify admins on suspicious patterns detected in audit logs.

**Anomaly Detection Rules**:

1. **Geographic Mismatch**:
   - Trigger: Login from different country within 1 hour
   - Action: Email admin + require MFA re-verification

2. **Brute Force Detection**:
   - Trigger: 5+ failed logins from same IP in 5 minutes
   - Action: Temporarily block IP (15 min) + email admin

3. **Unusual Hours**:
   - Trigger: Admin login outside normal business hours
   - Action: Log warning + optional email notification

4. **Mass Permission Changes**:
   - Trigger: 10+ role assignments in 5 minutes by single admin
   - Action: Email security team for review

5. **Multiple Concurrent Sessions**:
   - Trigger: User logged in from 5+ different IPs simultaneously
   - Action: Flag for review

**Notification Channels**:
- Email
- Slack webhook (optional)
- SMS (critical alerts only)

**Acceptance Criteria**:
- ✅ Configurable alert thresholds
- ✅ Real-time anomaly detection
- ✅ Email notifications sent to admins
- ✅ False positive rate <5%

---

### 5.5 Password Management Requirements

#### FR-010: Password Reset (Priority: HIGH)

**Description**: Self-service password reset via email link with secure token expiry.

**Processing Flow**:

**1. Request Password Reset**:

**Input**:
```json
{
  "email": "john.doe@example.com"
}
```

**Steps**:
1. Verify email exists (don't reveal if not)
2. Generate secure reset token (JWT, 1-hour expiry)
3. Store token hash in database with expiry
4. Send email with reset link: `https://app.com/reset?token=<token>`
5. Log reset request in audit logs

**2. Complete Password Reset**:

**Input**:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "new_password": "NewSecureP@ssw0rd456"
}
```

**Steps**:
1. Validate token signature and expiry
2. Validate new password strength
3. Check new password not in last 5 passwords (history)
4. Hash new password with Argon2id
5. Update `password_hash` in database
6. Invalidate all active sessions (logout everywhere)
7. Send confirmation email
8. Log password change in audit logs

**Outputs**:
```json
{
  "success": true,
  "message": "Password updated successfully. Please login with your new password."
}
```

**Acceptance Criteria**:
- ✅ Reset link expires in 1 hour
- ✅ Token can only be used once
- ✅ Old password is invalidated
- ✅ New password strength validated (min 12 chars, complexity)
- ✅ New password cannot match last 5 passwords
- ✅ All active sessions invalidated
- ✅ Confirmation email sent
- ✅ Audit log created

**Security Requirements**:
- Reset tokens are cryptographically secure (256-bit entropy)
- Tokens are single-use (invalidated after use)
- Rate limit: 3 reset requests per email per hour
- Don't reveal whether email exists in system

---

#### FR-011: Password Policy Enforcement (Priority: MEDIUM)

**Description**: Global password policies for complexity and rotation.

**Password Requirements**:

**Complexity**:
- Minimum length: 12 characters
- Must contain:
  - At least 1 uppercase letter (A-Z)
  - At least 1 lowercase letter (a-z)
  - At least 1 digit (0-9)
  - At least 1 special character (!@#$%^&*()_+-=[]{}|;:,.<>?)
- Maximum length: 128 characters
- No common passwords (check against list of 10k most common)

**History**:
- Cannot reuse last 5 passwords
- Store password hashes in `password_history` table

**Rotation** (Optional):
- Reminder to change password annually
- Grace period: 7 days after reminder
- Optional enforcement (configurable)

**Validation Response**:
```json
{
  "valid": false,
  "errors": [
    "Password must be at least 12 characters",
    "Password must contain at least one uppercase letter",
    "Password is in list of common passwords"
  ],
  "strength": {
    "score": 2,
    "label": "weak",
    "suggestions": [
      "Add more special characters",
      "Avoid common words"
    ]
  }
}
```

**Acceptance Criteria**:
- ✅ Password complexity enforced on creation and reset
- ✅ Last 5 passwords cannot be reused
- ✅ Common passwords rejected
- ✅ Annual rotation reminder (optional enforcement)
- ✅ Clear error messages for failed validation
- ✅ Password strength indicator provided

---

## 9. Database Schema Specifications

This section provides exhaustive database schema details from the domain documentation, including MongoDB collections, field specifications, validations, indexes, and relationships.

### 9.1 General Schema Guidelines

**Database Type**: MongoDB (Primary) with optional PostgreSQL (Secondary)

**Naming Conventions**:
- Collections/Tables: Plural, snake_case (e.g., `users`, `roles`)
- Fields: snake_case, descriptive (e.g., `first_name`, `created_at`)
- IDs: UUIDv4 for uniqueness (MongoDB: store as string or Binary UUID)

**Common Validations**:
- All timestamps: UTC timezone (ISO 8601 format)
- Strings: Trim whitespace, sanitize for XSS
- Enums: Use validation schemas in MongoDB

**Security**:
- Password hashes: Argon2id with `m=19456, t=2, p=1`
- Encryption: At-rest encryption enabled in MongoDB
- Access: Role-based access control in application layer

**Indexing Strategy**:
- Unique indexes on email, username
- Compound indexes for common queries
- TTL indexes for temporary data (e.g., reset tokens)

**Migration Notes**:
- Use MongoDB migrations library or custom scripts
- Version migrations for rollback capability
- Test migrations on staging before production

---

### 9.2 Users Collection

**Purpose**: Core storage for user identities, credentials, and profiles.

**MongoDB Schema**:
```javascript
{
  _id: UUID,
  username: String,
  email: String,
  password_hash: String,
  first_name: String,
  last_name: String,
  status: String,  // enum: 'active', 'inactive', 'suspended'
  created_at: ISODate,
  updated_at: ISODate,
  last_login: ISODate,  // nullable
  phone_number: String,  // nullable
  profile_picture_url: String,  // nullable
  email_verified: Boolean,
  deleted_at: ISODate,  // nullable (soft delete)
  metadata: {
    suspension_reason: String,  // if suspended
    suspended_until: ISODate,   // if suspended
    failed_login_attempts: Number,
    account_locked_until: ISODate
  }
}
```

**Field Specifications**:

| Field | Data Type | Constraints | Description |
|-------|-----------|-------------|-------------|
| `_id` | UUID (String) | PRIMARY KEY, NOT NULL, UNIQUE | Globally unique user identifier (UUIDv4) |
| `username` | String | UNIQUE, NOT NULL, MIN_LEN=3, MAX_LEN=50, REGEX=/^[a-zA-Z0-9_-]+$/ | Login handle; alphanumeric + underscores/hyphens |
| `email` | String | UNIQUE, NOT NULL, VALID_EMAIL_REGEX, LOWERCASE_NORMALIZED | Primary login/contact; normalized to lowercase |
| `password_hash` | String | NOT NULL, MIN_LEN=60 | Argon2id hash; never query/return directly |
| `first_name` | String | NOT NULL, MAX_LEN=100, REGEX=/^[a-zA-Z\s'-]+$/ | User's given name; allow apostrophes/hyphens/spaces |
| `last_name` | String | NOT NULL, MAX_LEN=100, REGEX=/^[a-zA-Z\s'-]+$/ | User's surname |
| `status` | String (Enum) | NOT NULL, DEFAULT='active', VALUES=['active','inactive','suspended'] | Account state |
| `created_at` | ISODate | NOT NULL, DEFAULT=NOW() | Account creation time; immutable |
| `updated_at` | ISODate | NOT NULL, DEFAULT=NOW(), AUTO_UPDATE | Last modification time |
| `last_login` | ISODate | NULLABLE | Most recent successful login |
| `phone_number` | String | NULLABLE, REGEX=/^\+?[1-9]\d{1,14}$/ | E.164 format |
| `profile_picture_url` | String | NULLABLE, MAX_LEN=500, VALID_URL | CDN/S3 URL |
| `email_verified` | Boolean | NOT NULL, DEFAULT=false | Email verification status |
| `deleted_at` | ISODate | NULLABLE | Soft delete timestamp |

**Indexes**:
```javascript
db.users.createIndex({ email: 1 }, { unique: true })
db.users.createIndex({ username: 1 }, { unique: true })
db.users.createIndex({ status: 1 })
db.users.createIndex({ created_at: -1 })
db.users.createIndex({ last_login: -1 })
```

**Validation Schema** (MongoDB):
```javascript
db.createCollection("users", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "username", "email", "password_hash", "first_name", "last_name", "status", "created_at", "updated_at"],
      properties: {
        _id: { bsonType: "string", pattern: "^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$" },
        username: { bsonType: "string", minLength: 3, maxLength: 50, pattern: "^[a-zA-Z0-9_-]+$" },
        email: { bsonType: "string", pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$" },
        password_hash: { bsonType: "string", minLength: 60 },
        first_name: { bsonType: "string", maxLength: 100 },
        last_name: { bsonType: "string", maxLength: 100 },
        status: { enum: ["active", "inactive", "suspended"] },
        created_at: { bsonType: "date" },
        updated_at: { bsonType: "date" },
        last_login: { bsonType: ["date", "null"] },
        phone_number: { bsonType: ["string", "null"], pattern: "^\\+?[1-9]\\d{1,14}$" },
        profile_picture_url: { bsonType: ["string", "null"], maxLength: 500 },
        email_verified: { bsonType: "bool" },
        deleted_at: { bsonType: ["date", "null"] }
      }
    }
  }
})
```

**Relationships**:
- One-to-Many: To `user_roles` (user can have multiple roles)
- One-to-One: To `user_settings` (user preferences)
- One-to-Many: To `audit_logs` (user actions)
- One-to-Many: To `sessions` (active sessions)

**Implementation Notes**:
- Query optimization: Use `email` index for logins
- Pagination: Use `created_at` index with limit/skip
- Soft delete: Filter `deleted_at: null` in queries
- Case-insensitive search: Use text indexes for username/email

---

### 9.3 Roles Collection

**Purpose**: Defines reusable access groups for RBAC.

**MongoDB Schema**:
```javascript
{
  _id: String,  // role identifier (e.g., "admin", "editor", "user")
  name: String,
  description: String,
  created_at: ISODate,
  updated_at: ISODate,
  hierarchy_level: Number,  // for hierarchical roles (higher = more privilege)
  is_system_role: Boolean   // prevent deletion of system roles
}
```

**Field Specifications**:

| Field | Data Type | Constraints | Description |
|-------|-----------|-------------|-------------|
| `_id` | String | PRIMARY KEY, NOT NULL, UNIQUE | Role identifier (lowercase, e.g., "admin") |
| `name` | String | UNIQUE, NOT NULL, MAX_LEN=50 | Display name for role |
| `description` | String | NULLABLE, MAX_LEN=1000 | Optional scope explanation |
| `created_at` | ISODate | NOT NULL, DEFAULT=NOW() | Creation timestamp |
| `updated_at` | ISODate | NOT NULL, DEFAULT=NOW() | Last update timestamp |
| `hierarchy_level` | Number | NOT NULL, DEFAULT=0 | Hierarchy level (admin=100, editor=50, user=0) |
| `is_system_role` | Boolean | NOT NULL, DEFAULT=false | Prevent deletion of system roles |

**Indexes**:
```javascript
db.roles.createIndex({ name: 1 }, { unique: true })
db.roles.createIndex({ hierarchy_level: -1 })
```

**Seed Data**:
```javascript
db.roles.insertMany([
  {
    _id: "admin",
    name: "Administrator",
    description: "Full system access with all permissions",
    hierarchy_level: 100,
    is_system_role: true,
    created_at: new Date(),
    updated_at: new Date()
  },
  {
    _id: "editor",
    name: "Editor",
    description: "Can create and manage content",
    hierarchy_level: 50,
    is_system_role: true,
    created_at: new Date(),
    updated_at: new Date()
  },
  {
    _id: "user",
    name: "User",
    description: "Basic user access",
    hierarchy_level: 0,
    is_system_role: true,
    created_at: new Date(),
    updated_at: new Date()
  }
])
```

**Relationships**:
- Many-to-Many: To `users` via `user_roles` junction
- Many-to-Many: To `permissions` via `role_permissions` junction

---

### 9.4 Permissions Collection

**Purpose**: Granular action definitions for fine-grained access control.

**MongoDB Schema**:
```javascript
{
  _id: String,  // permission identifier (e.g., "read:users")
  name: String,
  description: String,
  resource: String,
  action: String,
  created_at: ISODate,
  updated_at: ISODate
}
```

**Field Specifications**:

| Field | Data Type | Constraints | Description |
|-------|-----------|-------------|-------------|
| `_id` | String | PRIMARY KEY, NOT NULL, UNIQUE | Permission identifier (format: "action:resource") |
| `name` | String | UNIQUE, NOT NULL, PATTERN=/^[a-z_]+:[a-z_]+$/ | e.g., "read:users", "write:posts" |
| `description` | String | NULLABLE, MAX_LEN=500 | What the permission allows |
| `resource` | String | NULLABLE, MAX_LEN=50 | Resource category (e.g., "users") |
| `action` | String | NULLABLE, MAX_LEN=50 | Action type (e.g., "read", "write", "delete") |
| `created_at` | ISODate | NOT NULL | Creation timestamp |
| `updated_at` | ISODate | NOT NULL | Last update timestamp |

**Indexes**:
```javascript
db.permissions.createIndex({ name: 1 }, { unique: true })
db.permissions.createIndex({ resource: 1 })
db.permissions.createIndex({ action: 1 })
```

**Seed Data**:
```javascript
db.permissions.insertMany([
  // User management permissions
  { _id: "read:users", name: "read:users", description: "Read user profiles", resource: "users", action: "read", created_at: new Date(), updated_at: new Date() },
  { _id: "create:users", name: "create:users", description: "Create new users", resource: "users", action: "create", created_at: new Date(), updated_at: new Date() },
  { _id: "update:users", name: "update:users", description: "Update user profiles", resource: "users", action: "update", created_at: new Date(), updated_at: new Date() },
  { _id: "delete:users", name: "delete:users", description: "Delete user accounts", resource: "users", action: "delete", created_at: new Date(), updated_at: new Date() },

  // Role management permissions
  { _id: "read:roles", name: "read:roles", description: "View roles and permissions", resource: "roles", action: "read", created_at: new Date(), updated_at: new Date() },
  { _id: "assign_role", name: "assign_role", description: "Assign roles to users", resource: "roles", action: "assign", created_at: new Date(), updated_at: new Date() },
  { _id: "create:roles", name: "create:roles", description: "Create new roles", resource: "roles", action: "create", created_at: new Date(), updated_at: new Date() },

  // Audit log permissions
  { _id: "read:audit_logs", name: "read:audit_logs", description: "View audit logs", resource: "audit_logs", action: "read", created_at: new Date(), updated_at: new Date() },
  { _id: "export:audit_logs", name: "export:audit_logs", description: "Export audit logs", resource: "audit_logs", action: "export", created_at: new Date(), updated_at: new Date() }
])
```

**Relationships**:
- Many-to-Many: To `roles` via `role_permissions` junction

---

### 9.5 User_Roles Junction Collection

**Purpose**: Many-to-many relationship between users and roles.

**MongoDB Schema**:
```javascript
{
  _id: UUID,
  user_id: UUID,
  role_id: String,
  assigned_at: ISODate,
  assigned_by: UUID,  // admin who assigned the role
  expires_at: ISODate  // nullable, for temporary role assignments
}
```

**Indexes**:
```javascript
db.user_roles.createIndex({ user_id: 1, role_id: 1 }, { unique: true })
db.user_roles.createIndex({ user_id: 1 })
db.user_roles.createIndex({ role_id: 1 })
db.user_roles.createIndex({ expires_at: 1 }, { expireAfterSeconds: 0 })  // TTL index
```

**Query Examples**:

Get user's roles:
```javascript
db.user_roles.find({ user_id: "550e8400-e29b-41d4-a716-446655440000" })
```

Get users with specific role:
```javascript
db.user_roles.find({ role_id: "admin" })
```

---

### 9.6 Role_Permissions Junction Collection

**Purpose**: Many-to-many relationship between roles and permissions.

**MongoDB Schema**:
```javascript
{
  _id: UUID,
  role_id: String,
  permission_id: String,
  granted_at: ISODate,
  granted_by: UUID
}
```

**Indexes**:
```javascript
db.role_permissions.createIndex({ role_id: 1, permission_id: 1 }, { unique: true })
db.role_permissions.createIndex({ role_id: 1 })
db.role_permissions.createIndex({ permission_id: 1 })
```

**Seed Data** (Admin role gets all permissions):
```javascript
const adminPermissions = [
  "read:users", "create:users", "update:users", "delete:users",
  "read:roles", "create:roles", "assign_role",
  "read:audit_logs", "export:audit_logs"
]

adminPermissions.forEach(perm => {
  db.role_permissions.insert({
    _id: UUID(),
    role_id: "admin",
    permission_id: perm,
    granted_at: new Date(),
    granted_by: null
  })
})
```

---

### 9.7 User_Settings Collection

**Purpose**: Per-user preferences and settings.

**MongoDB Schema**:
```javascript
{
  _id: UUID,
  user_id: UUID,  // unique, one-to-one with users
  settings: {
    theme: String,  // "light", "dark", "auto"
    language: String,  // "en", "id", etc.
    notifications: {
      email: Boolean,
      sms: Boolean,
      push: Boolean
    },
    timezone: String,  // "UTC", "Asia/Jakarta", etc.
    date_format: String,  // "YYYY-MM-DD", "DD/MM/YYYY", etc.
    two_factor_enabled: Boolean,
    two_factor_method: String  // "totp", "sms", null
  },
  updated_at: ISODate
}
```

**Indexes**:
```javascript
db.user_settings.createIndex({ user_id: 1 }, { unique: true })
```

**Default Settings**:
```javascript
{
  theme: "light",
  language: "en",
  notifications: {
    email: true,
    sms: false,
    push: false
  },
  timezone: "UTC",
  date_format: "YYYY-MM-DD",
  two_factor_enabled: false,
  two_factor_method: null
}
```

---

### 9.8 Audit_Logs Collection

**Purpose**: Immutable record of all user events for compliance and security.

**MongoDB Schema**:
```javascript
db.createCollection("audit_logs", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "action", "timestamp"],
      properties: {
        _id: {
          bsonType: "string",
          pattern: "^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$",
          description: "UUID v4 primary key"
        },
        user_id: {
          bsonType: ["string", "null"],
          pattern: "^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|null)$",
          description: "User who performed action (null for system events)"
        },
        action: {
          bsonType: "string",
          minLength: 3,
          maxLength: 100,
          description: "Event type (e.g., 'login_success', 'user_created', 'permission_granted')"
        },
        details: {
          bsonType: ["object", "null"],
          description: "Event-specific data (flexible JSON)"
        },
        ip_address: {
          bsonType: ["string", "null"],
          maxLength: 45,
          description: "Client IP address (IPv4 or IPv6)"
        },
        user_agent: {
          bsonType: ["string", "null"],
          maxLength: 500,
          description: "Browser/client user agent string"
        },
        timestamp: {
          bsonType: "date",
          description: "Event timestamp (UTC, auto-generated)"
        },
        session_id: {
          bsonType: ["string", "null"],
          pattern: "^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|null)$",
          description: "Associated session ID (nullable)"
        },
        resource_type: {
          bsonType: ["string", "null"],
          enum: [null, "user", "role", "permission", "session", "audit_log", "user_permission"],
          description: "Type of resource affected"
        },
        resource_id: {
          bsonType: ["string", "null"],
          maxLength: 255,
          description: "ID of affected resource"
        },
        severity: {
          bsonType: "string",
          enum: ["info", "warning", "error", "critical"],
          description: "Log severity level (default: info)"
        },
        archived: {
          bsonType: "bool",
          description: "Whether log has been archived to external storage (default: false)"
        },
        archived_at: {
          bsonType: ["date", "null"],
          description: "Timestamp when log was archived (null if not archived)"
        },
        archive_location: {
          bsonType: ["string", "null"],
          maxLength: 500,
          description: "S3/storage path where log is archived (e.g., 's3://logs/2025/01/audit_logs_20250119.json.gz')"
        }
      }
    }
  }
})
```

**Field Specifications**:

| Field | Data Type | Constraints | Description |
|-------|-----------|-------------|-------------|
| `_id` | UUID | PRIMARY KEY, NOT NULL | Unique log entry ID |
| `user_id` | UUID | NULLABLE, INDEXED | User who performed action (null for system) |
| `action` | String | NOT NULL, INDEXED, 3-100 chars | Event type (e.g., "login_success") |
| `details` | Object | NULLABLE | Event-specific data (JSON) |
| `ip_address` | String | NULLABLE, MAX 45 chars | Client IP address (IPv4/IPv6) |
| `user_agent` | String | NULLABLE, MAX 500 chars | Browser/client info |
| `timestamp` | ISODate | NOT NULL, INDEXED | Event timestamp (UTC) |
| `session_id` | UUID | NULLABLE | Associated session |
| `resource_type` | String | NULLABLE, ENUM | Type of resource affected |
| `resource_id` | String | NULLABLE, MAX 255 chars | ID of affected resource |
| `severity` | String | NOT NULL, ENUM, DEFAULT 'info' | Log severity level |
| `archived` | Boolean | NOT NULL, DEFAULT false | Archive status flag |
| `archived_at` | ISODate | NULLABLE | Archive timestamp |
| `archive_location` | String | NULLABLE, MAX 500 chars | External storage path |

**Indexes**:
```javascript
// Primary queries: user activity timeline
db.audit_logs.createIndex({ user_id: 1, timestamp: -1 })

// Action-based queries: find all login attempts
db.audit_logs.createIndex({ action: 1, timestamp: -1 })

// Time-based queries: recent activity
db.audit_logs.createIndex({ timestamp: -1 })

// Security queries: track IP activity
db.audit_logs.createIndex({ ip_address: 1, timestamp: -1 })

// Resource queries: audit trail for specific resource
db.audit_logs.createIndex({ resource_type: 1, resource_id: 1, timestamp: -1 })

// Severity-based queries: find critical events
db.audit_logs.createIndex({ severity: 1, timestamp: -1 })

// Archival job: find logs ready for archival
db.audit_logs.createIndex({
  archived: 1,
  timestamp: 1
}, {
  partialFilterExpression: { archived: false }
})

// Cleanup job: find archived logs older than retention period
db.audit_logs.createIndex({
  archived: 1,
  archived_at: 1
}, {
  partialFilterExpression: { archived: true }
})
```

---

## 9.8.1 Audit Log Management Strategy (Enable/Disable, Archival, Cleanup)

### **Overview: Hybrid 3-Tier Approach**

The audit log management system implements three complementary mechanisms:

1. **Real-time Control**: Enable/disable logging dynamically via feature flags
2. **Automated Archival**: Export old logs to cheap external storage (S3, GCS, Azure Blob)
3. **Automated Cleanup**: Delete archived logs from MongoDB after retention period

**Architecture Diagram**:
```
┌─────────────────────────────────────────────────────────────┐
│                    Audit Event Occurs                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
        ┌──────────────────────────────┐
        │  Check Feature Flags         │
        │  - Is audit logging enabled? │
        │  - Is this action excluded?  │
        └──────────┬───────────────────┘
                   │ (if enabled)
                   ▼
        ┌──────────────────────────────┐
        │  Write to MongoDB            │
        │  audit_logs collection       │
        │  (Hot storage: 0-90 days)    │
        └──────────┬───────────────────┘
                   │
                   ▼
        ┌──────────────────────────────┐
        │  Daily Archival Job          │
        │  (runs at 00:00 UTC)         │
        │  - Export logs >90 days old  │
        │  - Compress to JSON.gz       │
        │  - Upload to S3/GCS          │
        │  - Mark as archived=true     │
        └──────────┬───────────────────┘
                   │
                   ▼
        ┌──────────────────────────────┐
        │  Weekly Cleanup Job          │
        │  (runs Sunday 00:00 UTC)     │
        │  - Delete archived logs      │
        │    >30 days after archival   │
        │  - Keep MongoDB lean         │
        └──────────────────────────────┘
                   │
                   ▼
        ┌──────────────────────────────┐
        │  S3/GCS Long-term Storage    │
        │  (Cold storage: 7 years)     │
        │  - Compliance retention      │
        │  - Compressed archives       │
        │  - Lifecycle policy to       │
        │    Glacier after 1 year      │
        └──────────────────────────────┘
```

---

### **1. Feature Flags: Enable/Disable Audit Logging**

**Implementation**: Use `user_settings` collection or dedicated `system_settings` collection.

**Schema for System Settings**:
```javascript
db.createCollection("system_settings", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "key", "value", "updated_at"],
      properties: {
        _id: { bsonType: "string" },
        key: { bsonType: "string" },
        value: { bsonType: ["object", "bool", "string", "number"] },
        description: { bsonType: "string" },
        updated_at: { bsonType: "date" },
        updated_by: { bsonType: ["string", "null"] }
      }
    }
  }
})

// Example: Master audit logging toggle
{
  _id: "audit_logging_enabled",
  key: "audit_logging_enabled",
  value: true,  // Set to false to disable ALL audit logging
  description: "Master switch for audit logging system",
  updated_at: ISODate("2025-01-19T10:00:00Z"),
  updated_by: "admin-user-id"
}

// Example: Granular action-based controls
{
  _id: "audit_excluded_actions",
  key: "audit_excluded_actions",
  value: {
    "read:users": true,        // Exclude read operations (high volume, low value)
    "health_check": true,       // Exclude health checks
    "list:sessions": true       // Exclude session listings
  },
  description: "Actions to exclude from audit logging",
  updated_at: ISODate("2025-01-19T10:00:00Z"),
  updated_by: "admin-user-id"
}

// Example: User-type based controls
{
  _id: "audit_excluded_user_types",
  key: "audit_excluded_user_types",
  value: {
    "service_account": true,    // Exclude service account actions
    "read_only_user": true      // Exclude read-only users
  },
  description: "User types to exclude from audit logging",
  updated_at: ISODate("2025-01-19T10:00:00Z"),
  updated_by: "admin-user-id"
}
```

**Rust Implementation Example**:
```rust
// apps/sapiens/src/infrastructure/audit/audit_logger.rs

pub struct AuditLogger {
    mongo_client: MongoClient,
    settings_cache: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl AuditLogger {
    pub async fn log_event(&self, event: AuditEvent) -> Result<()> {
        // 1. Check master switch
        if !self.is_audit_logging_enabled().await? {
            tracing::debug!("Audit logging is disabled globally");
            return Ok(()); // Skip logging
        }

        // 2. Check if action is excluded
        if self.is_action_excluded(&event.action).await? {
            tracing::debug!("Action {} is excluded from audit logging", event.action);
            return Ok(());
        }

        // 3. Check if user type is excluded
        if let Some(user_id) = &event.user_id {
            if self.is_user_type_excluded(user_id).await? {
                tracing::debug!("User type excluded from audit logging");
                return Ok(());
            }
        }

        // 4. Write to MongoDB
        self.write_audit_log(event).await?;
        Ok(())
    }

    async fn is_audit_logging_enabled(&self) -> Result<bool> {
        let settings = self.settings_cache.read().await;
        Ok(settings
            .get("audit_logging_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)) // Default: enabled
    }

    async fn is_action_excluded(&self, action: &str) -> Result<bool> {
        let settings = self.settings_cache.read().await;
        Ok(settings
            .get("audit_excluded_actions")
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get(action))
            .and_then(|v| v.as_bool())
            .unwrap_or(false)) // Default: not excluded
    }
}
```

**Admin API Endpoints**:
```
POST /api/v1/admin/audit/settings/enable
POST /api/v1/admin/audit/settings/disable
POST /api/v1/admin/audit/settings/exclude-action
POST /api/v1/admin/audit/settings/include-action
GET  /api/v1/admin/audit/settings
```

---

### **2. Automated Archival to External Storage**

**Retention Strategy**:
- **Hot storage (MongoDB)**: Last 90 days (fast queries, recent activity)
- **Cold storage (S3/GCS)**: 90 days - 7 years (compliance, cheap storage)
- **Glacier/Deep Archive**: 7+ years (compliance, very cheap)

**Daily Archival Job** (runs at 00:00 UTC):

**Rust Implementation**:
```rust
// apps/sapiens/src/infrastructure/jobs/audit_archival_job.rs

use aws_sdk_s3::Client as S3Client;
use flate2::write::GzEncoder;
use flate2::Compression;

pub struct AuditArchivalJob {
    mongo_client: MongoClient,
    s3_client: S3Client,
    bucket_name: String,
}

impl AuditArchivalJob {
    pub async fn run(&self) -> Result<ArchivalReport> {
        let cutoff_date = Utc::now() - Duration::days(90);

        // 1. Find unarchived logs older than 90 days
        let logs_to_archive = self.find_logs_to_archive(cutoff_date).await?;

        if logs_to_archive.is_empty() {
            tracing::info!("No logs to archive");
            return Ok(ArchivalReport::default());
        }

        // 2. Group by date for efficient storage
        let grouped_logs = self.group_logs_by_date(logs_to_archive);

        let mut total_archived = 0;
        let mut total_size_bytes = 0;

        for (date, logs) in grouped_logs {
            // 3. Serialize to JSON
            let json_data = serde_json::to_string(&logs)?;

            // 4. Compress with gzip
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(json_data.as_bytes())?;
            let compressed_data = encoder.finish()?;

            // 5. Upload to S3
            let s3_key = format!(
                "audit_logs/{}/{:02}/audit_logs_{}.json.gz",
                date.year(),
                date.month(),
                date.format("%Y%m%d")
            );

            self.s3_client
                .put_object()
                .bucket(&self.bucket_name)
                .key(&s3_key)
                .body(compressed_data.into())
                .content_type("application/gzip")
                .metadata("original_count", logs.len().to_string())
                .send()
                .await?;

            // 6. Mark logs as archived in MongoDB
            let log_ids: Vec<String> = logs.iter().map(|l| l.id.clone()).collect();

            self.mongo_client
                .database("sapiens")
                .collection::<AuditLog>("audit_logs")
                .update_many(
                    doc! { "_id": { "$in": log_ids } },
                    doc! {
                        "$set": {
                            "archived": true,
                            "archived_at": Utc::now(),
                            "archive_location": format!("s3://{}/{}", self.bucket_name, s3_key)
                        }
                    },
                    None,
                )
                .await?;

            total_archived += logs.len();
            total_size_bytes += compressed_data.len();

            tracing::info!(
                "Archived {} logs for {} to {} ({} KB compressed)",
                logs.len(),
                date,
                s3_key,
                compressed_data.len() / 1024
            );
        }

        Ok(ArchivalReport {
            total_archived,
            total_size_bytes,
            compression_ratio: calculate_compression_ratio(total_size_bytes, json_data.len()),
        })
    }

    async fn find_logs_to_archive(&self, cutoff_date: DateTime<Utc>) -> Result<Vec<AuditLog>> {
        self.mongo_client
            .database("sapiens")
            .collection::<AuditLog>("audit_logs")
            .find(
                doc! {
                    "archived": false,
                    "timestamp": { "$lt": cutoff_date }
                },
                None,
            )
            .await?
            .try_collect()
            .await
    }
}
```

**Cron Schedule**:
```bash
# /etc/cron.d/sapiens-audit-archival
0 0 * * * sapiens /usr/local/bin/sapiens-cli audit-archival-job
```

**S3 Lifecycle Policy** (Move to Glacier after 1 year):
```json
{
  "Rules": [
    {
      "Id": "MoveAuditLogsToGlacier",
      "Status": "Enabled",
      "Prefix": "audit_logs/",
      "Transitions": [
        {
          "Days": 365,
          "StorageClass": "GLACIER"
        },
        {
          "Days": 2555,
          "StorageClass": "DEEP_ARCHIVE"
        }
      ],
      "Expiration": {
        "Days": 2555
      }
    }
  ]
}
```

---

### **3. Automated Cleanup from MongoDB**

**Weekly Cleanup Job** (runs Sunday 00:00 UTC):

**Rust Implementation**:
```rust
// apps/sapiens/src/infrastructure/jobs/audit_cleanup_job.rs

pub struct AuditCleanupJob {
    mongo_client: MongoClient,
}

impl AuditCleanupJob {
    pub async fn run(&self) -> Result<CleanupReport> {
        // Delete archived logs older than 30 days after archival
        let cutoff_date = Utc::now() - Duration::days(30);

        let result = self.mongo_client
            .database("sapiens")
            .collection::<AuditLog>("audit_logs")
            .delete_many(
                doc! {
                    "archived": true,
                    "archived_at": { "$lt": cutoff_date }
                },
                None,
            )
            .await?;

        tracing::info!(
            "Cleanup job completed: {} archived logs deleted from MongoDB",
            result.deleted_count
        );

        Ok(CleanupReport {
            deleted_count: result.deleted_count,
            cutoff_date,
        })
    }
}
```

**Cron Schedule**:
```bash
# /etc/cron.d/sapiens-audit-cleanup
0 0 * * 0 sapiens /usr/local/bin/sapiens-cli audit-cleanup-job
```

---

### **4. Querying Archived Logs**

**Admin Tool to Search Archived Logs**:

```rust
// apps/sapiens/src/infrastructure/audit/archived_log_query.rs

pub struct ArchivedLogQuery {
    s3_client: S3Client,
    bucket_name: String,
}

impl ArchivedLogQuery {
    pub async fn search(&self, query: SearchQuery) -> Result<Vec<AuditLog>> {
        // 1. Determine which archive files to check based on date range
        let s3_keys = self.get_relevant_archive_keys(&query.date_range).await?;

        let mut results = Vec::new();

        for key in s3_keys {
            // 2. Download from S3
            let obj = self.s3_client
                .get_object()
                .bucket(&self.bucket_name)
                .key(&key)
                .send()
                .await?;

            // 3. Decompress
            let compressed_data = obj.body.collect().await?.into_bytes();
            let mut decoder = GzDecoder::new(&compressed_data[..]);
            let mut json_data = String::new();
            decoder.read_to_string(&mut json_data)?;

            // 4. Deserialize and filter
            let logs: Vec<AuditLog> = serde_json::from_str(&json_data)?;
            let filtered: Vec<AuditLog> = logs.into_iter()
                .filter(|log| query.matches(log))
                .collect();

            results.extend(filtered);

            if results.len() >= query.limit {
                break;
            }
        }

        Ok(results.into_iter().take(query.limit).collect())
    }
}
```

**Admin API Endpoint**:
```
POST /api/v1/admin/audit/search-archived
{
  "date_range": {
    "start": "2024-01-01T00:00:00Z",
    "end": "2024-12-31T23:59:59Z"
  },
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "action": "user_deleted",
  "limit": 100
}
```

---

### **5. Configuration Summary**

**Environment Variables**:
```bash
# Feature Flags
AUDIT_LOGGING_ENABLED=true
AUDIT_EXCLUDED_ACTIONS=read:users,health_check

# Archival Settings
AUDIT_ARCHIVAL_ENABLED=true
AUDIT_HOT_RETENTION_DAYS=90
AUDIT_ARCHIVAL_BUCKET=sapiens-audit-logs-prod
AUDIT_ARCHIVAL_REGION=us-east-1

# Cleanup Settings
AUDIT_CLEANUP_ENABLED=true
AUDIT_CLEANUP_AFTER_DAYS=30  # Delete from MongoDB 30 days after archival
```

**Default Retention Policy**:
- **MongoDB (Hot)**: 90 days
- **S3 Standard**: 90 days - 1 year
- **S3 Glacier**: 1 year - 7 years
- **S3 Deep Archive**: 7+ years (compliance)
- **Deletion**: After 7 years

---

### **6. Monitoring & Alerts**

**Metrics to Track**:
- Audit logs written per day
- Archival job success/failure rate
- Cleanup job deleted count
- MongoDB collection size
- S3 storage costs
- Query performance (hot vs archived)

**Alerts**:
- Archival job failed for 2+ consecutive days
- MongoDB audit_logs collection >100GB
- S3 upload failures
- Cleanup job deleted 0 records (unexpected)

---

### **7. Cost Optimization**

**MongoDB vs S3 Cost Comparison** (example):
- MongoDB Atlas (M30): $0.08/GB/month
- S3 Standard: $0.023/GB/month (3.5x cheaper)
- S3 Glacier: $0.004/GB/month (20x cheaper)
- S3 Deep Archive: $0.00099/GB/month (80x cheaper)

**Expected Savings**:
- 1M audit logs/day ≈ 500MB/day ≈ 15GB/month
- MongoDB only: 15GB × $0.08 = $1.20/month
- With archival (90-day hot, rest S3 Glacier):
  - Hot (3 months): 45GB × $0.08 = $3.60/month
  - Cold (9 months): 135GB × $0.004 = $0.54/month
  - **Total**: $4.14/month vs $14.40/month (71% savings for 1 year)

**Example Entries**:

Login success:
```javascript
{
  _id: UUID(),
  user_id: "550e8400-e29b-41d4-a716-446655440000",
  action: "login_success",
  details: {
    auth_method: "password",
    mfa_used: true
  },
  ip_address: "192.168.1.100",
  user_agent: "Mozilla/5.0...",
  timestamp: new Date(),
  session_id: UUID(),
  resource_type: "user",
  resource_id: "550e8400-e29b-41d4-a716-446655440000"
}
```

Failed login:
```javascript
{
  _id: UUID(),
  user_id: null,
  action: "login_failure",
  details: {
    email_or_username: "john.doe@example.com",
    reason: "invalid_password",
    failed_attempts: 3
  },
  ip_address: "203.0.113.50",
  user_agent: "curl/7.68.0",
  timestamp: new Date(),
  session_id: null,
  resource_type: "user",
  resource_id: null
}
```

**Querying Examples**:

Get user's recent activity:
```javascript
db.audit_logs.find({
  user_id: "550e8400-e29b-41d4-a716-446655440000"
}).sort({ timestamp: -1 }).limit(50)
```

Get failed login attempts from IP:
```javascript
db.audit_logs.find({
  action: "login_failure",
  ip_address: "203.0.113.50",
  timestamp: { $gte: new Date(Date.now() - 3600000) }  // last hour
})
```

---

### 9.9 Sessions Collection

**Purpose**: Track active user sessions with JWT token management.

**MongoDB Schema**:
```javascript
{
  _id: UUID,  // session ID
  user_id: UUID,
  access_token_jti: String,  // JWT ID for access token
  refresh_token_jti: String,  // JWT ID for refresh token
  created_at: ISODate,
  expires_at: ISODate,
  last_activity: ISODate,
  ip_address: String,
  user_agent: String,
  is_active: Boolean,
  revoked_at: ISODate,  // nullable
  revoked_reason: String  // nullable
}
```

**Indexes**:
```javascript
db.sessions.createIndex({ user_id: 1, is_active: 1 })
db.sessions.createIndex({ access_token_jti: 1 })
db.sessions.createIndex({ refresh_token_jti: 1 })
db.sessions.createIndex({ expires_at: 1 }, { expireAfterSeconds: 0 })  // TTL
```

**Operations**:

Create session:
```javascript
db.sessions.insert({
  _id: UUID(),
  user_id: "550e8400-e29b-41d4-a716-446655440000",
  access_token_jti: "at_abc123",
  refresh_token_jti: "rt_def456",
  created_at: new Date(),
  expires_at: new Date(Date.now() + 86400000),  // 24 hours
  last_activity: new Date(),
  ip_address: "192.168.1.100",
  user_agent: "Mozilla/5.0...",
  is_active: true,
  revoked_at: null,
  revoked_reason: null
})
```

Revoke session (logout):
```javascript
db.sessions.updateOne(
  { _id: sessionId },
  {
    $set: {
      is_active: false,
      revoked_at: new Date(),
      revoked_reason: "user_logout"
    }
  }
)
```

Revoke all user sessions (password change):
```javascript
db.sessions.updateMany(
  { user_id: userId, is_active: true },
  {
    $set: {
      is_active: false,
      revoked_at: new Date(),
      revoked_reason: "password_changed"
    }
  }
)
```

---

### 9.10 Password_Reset_Tokens Collection

**Purpose**: Temporary tokens for password reset flow.

**MongoDB Schema**:
```javascript
{
  _id: UUID,
  user_id: UUID,
  token_hash: String,  // SHA-256 hash of token
  created_at: ISODate,
  expires_at: ISODate,
  used_at: ISODate,  // nullable
  ip_address: String  // IP that requested reset
}
```

**Indexes**:
```javascript
db.password_reset_tokens.createIndex({ token_hash: 1 })
db.password_reset_tokens.createIndex({ user_id: 1, created_at: -1 })
db.password_reset_tokens.createIndex({ expires_at: 1 }, { expireAfterSeconds: 0 })  // TTL
```

**Token Generation**:
```javascript
const crypto = require('crypto')

// Generate secure random token
const token = crypto.randomBytes(32).toString('hex')  // 64 hex chars

// Hash token for storage (never store plain token)
const tokenHash = crypto.createHash('sha256').update(token).digest('hex')

// Store hash in database
db.password_reset_tokens.insert({
  _id: UUID(),
  user_id: userId,
  token_hash: tokenHash,
  created_at: new Date(),
  expires_at: new Date(Date.now() + 3600000),  // 1 hour
  used_at: null,
  ip_address: req.ip
})

// Send plain token to user via email (only once)
sendEmail(userEmail, `Reset link: https://app.com/reset?token=${token}`)
```

**Token Validation**:
```javascript
// Hash provided token
const providedHash = crypto.createHash('sha256').update(providedToken).digest('hex')

// Find token in database
const resetToken = db.password_reset_tokens.findOne({
  token_hash: providedHash,
  expires_at: { $gt: new Date() },
  used_at: null
})

if (!resetToken) {
  throw new Error('Invalid or expired reset token')
}

// Mark token as used
db.password_reset_tokens.updateOne(
  { _id: resetToken._id },
  { $set: { used_at: new Date() } }
)
```

---

### 9.11 MFA_Devices Collection

**Purpose**: Store MFA device configurations for users.

**MongoDB Schema**:
```javascript
{
  _id: UUID,
  user_id: UUID,
  device_type: String,  // "totp", "sms", "backup_codes"
  device_name: String,  // user-friendly name
  secret: String,  // encrypted TOTP secret or phone number
  backup_codes: [String],  // encrypted backup codes
  created_at: ISODate,
  last_used: ISODate,
  is_active: Boolean
}
```

**Indexes**:
```javascript
db.mfa_devices.createIndex({ user_id: 1, is_active: 1 })
db.mfa_devices.createIndex({ user_id: 1, device_type: 1 })
```

**TOTP Device Setup**:
```javascript
const speakeasy = require('speakeasy')

// Generate TOTP secret
const secret = speakeasy.generateSecret({
  name: 'Sapiens UMS (john.doe@example.com)',
  issuer: 'Sapiens'
})

// Store encrypted secret
db.mfa_devices.insert({
  _id: UUID(),
  user_id: userId,
  device_type: "totp",
  device_name: "Google Authenticator",
  secret: encrypt(secret.base32),  // encrypt before storage
  backup_codes: generateBackupCodes().map(encrypt),
  created_at: new Date(),
  last_used: null,
  is_active: true
})

// Return QR code and secret to user for enrollment
return {
  qr_code: secret.otpauth_url,
  manual_entry_key: secret.base32
}
```

---

### 9.12 User_Permissions Collection (Hybrid RBAC - Direct Permission Grants)

**Purpose**: Store direct permission grants to individual users for exception cases, temporary access, or resource-specific permissions. This implements the hybrid RBAC model where effective permissions = role-based permissions ∪ direct user permissions.

**MongoDB Schema**:
```javascript
db.createCollection("user_permissions", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "user_id", "permission_id", "granted_at", "granted_by", "reason", "is_active"],
      properties: {
        _id: {
          bsonType: "string",
          pattern: "^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$",
          description: "UUID v4 primary key"
        },
        user_id: {
          bsonType: "string",
          pattern: "^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$",
          description: "Foreign key to users collection"
        },
        permission_id: {
          bsonType: "string",
          minLength: 3,
          maxLength: 100,
          description: "Foreign key to permissions collection (e.g., 'delete:sensitive_records')"
        },
        granted_at: {
          bsonType: "date",
          description: "Timestamp when permission was granted (auto UTC)"
        },
        granted_by: {
          bsonType: "string",
          pattern: "^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$",
          description: "User ID of admin who granted the permission"
        },
        expires_at: {
          bsonType: ["date", "null"],
          description: "Optional expiry date for temporary access grants"
        },
        reason: {
          bsonType: "string",
          minLength: 10,
          maxLength: 500,
          description: "Business justification for direct permission grant (mandatory, immutable)"
        },
        resource_id: {
          bsonType: ["string", "null"],
          maxLength: 255,
          description: "Optional: scope permission to specific resource instance (e.g., 'project-123')"
        },
        resource_type: {
          bsonType: ["string", "null"],
          enum: [null, "users", "posts", "projects", "audit_records", "reports", "documents"],
          description: "Optional: type of resource if permission is resource-scoped"
        },
        is_active: {
          bsonType: "bool",
          description: "Whether grant is active (false = revoked)"
        },
        revoked_at: {
          bsonType: ["date", "null"],
          description: "Timestamp when permission was revoked (null if active)"
        },
        revoked_by: {
          bsonType: ["string", "null"],
          pattern: "^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|null)$",
          description: "User ID of admin who revoked the permission (null if active)"
        },
        revoked_reason: {
          bsonType: ["string", "null"],
          maxLength: 500,
          description: "Reason for revocation (null if active)"
        }
      }
    }
  }
})
```

**Indexes**:
```javascript
// Primary lookup: find all active permissions for a user
db.user_permissions.createIndex({
  user_id: 1,
  is_active: 1,
  expires_at: 1
})

// Query active permissions excluding expired
db.user_permissions.createIndex({
  user_id: 1,
  is_active: 1,
  expires_at: 1
}, {
  partialFilterExpression: {
    is_active: true,
    $or: [
      { expires_at: null },
      { expires_at: { $gte: new Date() } }
    ]
  }
})

// Duplicate prevention: ensure no active duplicate grants
db.user_permissions.createIndex(
  { user_id: 1, permission_id: 1, is_active: 1 },
  {
    unique: true,
    partialFilterExpression: { is_active: true }
  }
)

// Admin view: list all grants by permission
db.user_permissions.createIndex({ permission_id: 1, granted_at: -1 })

// Audit queries: find grants by admin
db.user_permissions.createIndex({ granted_by: 1, granted_at: -1 })

// Cleanup job: find expired permissions
db.user_permissions.createIndex({
  expires_at: 1
}, {
  partialFilterExpression: {
    expires_at: { $ne: null },
    is_active: true
  }
})

// Resource-scoped permission queries
db.user_permissions.createIndex({
  resource_type: 1,
  resource_id: 1,
  is_active: 1
})
```

**Example Documents**:

**1. Temporary Access Grant**:
```javascript
{
  _id: "550e8400-e29b-41d4-a716-446655440099",
  user_id: "550e8400-e29b-41d4-a716-446655440000",
  permission_id: "delete:audit_records",
  granted_at: ISODate("2025-01-19T10:30:00Z"),
  granted_by: "admin-550e8400-e29b-41d4-a716-446655440001",
  expires_at: ISODate("2025-12-31T23:59:59Z"),
  reason: "Temporary access for Q4 2025 audit cleanup project - approved by Legal dept ref #AUD-2025-047",
  resource_id: null,
  resource_type: null,
  is_active: true,
  revoked_at: null,
  revoked_by: null,
  revoked_reason: null
}
```

**2. Resource-Scoped Permission**:
```javascript
{
  _id: "550e8400-e29b-41d4-a716-446655440088",
  user_id: "550e8400-e29b-41d4-a716-446655440000",
  permission_id: "admin:project",
  granted_at: ISODate("2025-01-15T14:20:00Z"),
  granted_by: "admin-550e8400-e29b-41d4-a716-446655440001",
  expires_at: ISODate("2025-06-30T23:59:59Z"),
  reason: "Project lead for 'Phoenix Migration' - needs admin access to project-phoenix-2025 only",
  resource_id: "project-phoenix-2025",
  resource_type: "projects",
  is_active: true,
  revoked_at: null,
  revoked_by: null,
  revoked_reason: null
}
```

**3. Permanent Exception (No Expiry)**:
```javascript
{
  _id: "550e8400-e29b-41d4-a716-446655440077",
  user_id: "550e8400-e29b-41d4-a716-446655440000",
  permission_id: "read:sensitive_reports",
  granted_at: ISODate("2025-01-10T09:00:00Z"),
  granted_by: "admin-550e8400-e29b-41d4-a716-446655440001",
  expires_at: null,  // No expiry - requires quarterly review
  reason: "CFO role exception - requires access to financial reports not covered by standard CFO role - approved by CEO",
  resource_id: null,
  resource_type: null,
  is_active: true,
  revoked_at: null,
  revoked_by: null,
  revoked_reason: null
}
```

**4. Revoked Permission**:
```javascript
{
  _id: "550e8400-e29b-41d4-a716-446655440066",
  user_id: "550e8400-e29b-41d4-a716-446655440000",
  permission_id: "delete:users",
  granted_at: ISODate("2024-11-01T08:00:00Z"),
  granted_by: "admin-550e8400-e29b-41d4-a716-446655440001",
  expires_at: ISODate("2025-01-31T23:59:59Z"),
  reason: "Data cleanup project for legacy user migration - ticket #PROJ-1847",
  resource_id: null,
  resource_type: null,
  is_active: false,  // Revoked
  revoked_at: ISODate("2025-01-20T11:45:00Z"),
  revoked_by: "admin-550e8400-e29b-41d4-a716-446655440001",
  revoked_reason: "Project completed early - permission no longer needed"
}
```

**Query Examples**:

**1. Get All Active Permissions for User (Including Unexpired)**:
```javascript
db.user_permissions.find({
  user_id: "550e8400-e29b-41d4-a716-446655440000",
  is_active: true,
  $or: [
    { expires_at: null },
    { expires_at: { $gte: new Date() } }
  ]
})
```

**2. Get Effective Permissions (Role-Based + Direct)**:
```javascript
// See FR-006A Query Flow in Section 5 for complete aggregation pipeline
// This combines:
// - Permissions from user_roles → role_permissions → permissions
// - Direct permissions from user_permissions (active, not expired)
```

**3. Find Permissions Expiring Soon (Alert)**:
```javascript
const sevenDaysFromNow = new Date()
sevenDaysFromNow.setDate(sevenDaysFromNow.getDate() + 7)

db.user_permissions.find({
  is_active: true,
  expires_at: {
    $ne: null,
    $lte: sevenDaysFromNow,
    $gte: new Date()
  }
}).sort({ expires_at: 1 })
```

**4. Find All Permanent Grants Without Expiry (Governance Review)**:
```javascript
db.user_permissions.aggregate([
  {
    $match: {
      is_active: true,
      expires_at: null
    }
  },
  {
    $lookup: {
      from: "users",
      localField: "user_id",
      foreignField: "_id",
      as: "user"
    }
  },
  {
    $lookup: {
      from: "permissions",
      localField: "permission_id",
      foreignField: "_id",
      as: "permission"
    }
  },
  {
    $project: {
      _id: 1,
      user: { $arrayElemAt: ["$user", 0] },
      permission: { $arrayElemAt: ["$permission", 0] },
      granted_at: 1,
      reason: 1,
      days_since_granted: {
        $divide: [
          { $subtract: [new Date(), "$granted_at"] },
          1000 * 60 * 60 * 24
        ]
      }
    }
  },
  {
    $match: {
      days_since_granted: { $gte: 90 }  // Flag if >90 days old
    }
  }
])
```

**5. Cleanup Expired Permissions (Daily Job)**:
```javascript
// Mark expired permissions as inactive (soft delete)
db.user_permissions.updateMany(
  {
    is_active: true,
    expires_at: { $ne: null, $lt: new Date() }
  },
  {
    $set: {
      is_active: false,
      revoked_at: new Date(),
      revoked_by: "system",
      revoked_reason: "Automatic expiry - grant period ended"
    }
  }
)
```

**Validation Rules**:
- `reason` is mandatory (10-500 chars) and immutable - ensures audit trail
- `expires_at` must be in future when set (validated at application layer)
- Unique index prevents duplicate active grants for same user-permission pair
- `resource_type` must be valid enum value if specified
- `revoked_*` fields must be null if `is_active` is true

**Security Considerations**:
- All permission grants logged in `audit_logs` collection
- `reason` field is immutable - cannot be changed after grant
- Revocation requires separate admin permission (`revoke_permission`)
- Resource scoping prevents overly broad permission grants
- Expiry enforcement happens both at query time and via daily cleanup job

**Performance Optimization**:
- Partial indexes on `is_active` and `expires_at` reduce index size
- Compound indexes support common query patterns
- Unique partial index prevents duplicates without indexing revoked grants
- TTL index not used (soft delete preferred for audit trail)

**Governance & Best Practices**:
- **Direct permission grants should be <5% of total user base**
- Quarterly review of all permanent grants (expires_at = null)
- Monthly report of grants by permission type
- Alert administrators 7 days before expiry
- Auto-flag grants without expiry after 90 days for review
- Require detailed business justification in `reason` field
- Prefer role-based permissions for permanent access

---

## 19. Testing & Validation Framework

### 19.1 Testing Strategy

**Testing Pyramid**:
```
                /\
               /  \
              / E2E\
             /______\
            /        \
           /Integration\
          /____________ \
         /              \
        / Unit Tests     \
       /__________________\
```

**Test Coverage Goals**:
- **Unit Tests**: 95% code coverage
- **Integration Tests**: All API endpoints
- **E2E Tests**: Critical user flows
- **Security Tests**: OWASP Top 10
- **Performance Tests**: Load and stress testing

---

### 19.2 Unit Testing

**Scope**: Individual functions, value objects, domain logic

**Tools**:
- Rust: `cargo test`
- Test framework: Built-in Rust test framework
- Mocking: `mockall` crate

**Test Categories**:

**1. Domain Logic Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_with_valid_data() {
        let user = User::new(
            "john.doe@example.com",
            "john.doe",
            "SecureP@ssw0rd123"
        );
        assert!(user.is_ok());
    }

    #[test]
    fn test_user_creation_with_invalid_email() {
        let user = User::new(
            "invalid-email",
            "john.doe",
            "SecureP@ssw0rd123"
        );
        assert!(user.is_err());
    }

    #[test]
    fn test_password_hashing_uses_argon2() {
        let password = "SecureP@ssw0rd123";
        let hash = hash_password(password);
        assert!(hash.len() >= 60);
        assert!(verify_password(password, &hash));
    }
}
```

**2. Value Object Tests**:
```rust
#[test]
fn test_email_validation() {
    assert!(Email::new("valid@example.com").is_ok());
    assert!(Email::new("invalid-email").is_err());
    assert!(Email::new("@example.com").is_err());
    assert!(Email::new("user@").is_err());
}

#[test]
fn test_password_strength_validation() {
    assert!(Password::new("weak").is_err());
    assert!(Password::new("NoDigits!").is_err());
    assert!(Password::new("nospecial123").is_err());
    assert!(Password::new("SecureP@ssw0rd123").is_ok());
}
```

**3. Repository Tests** (with mocks):
```rust
#[tokio::test]
async fn test_user_repository_create() {
    let mock_db = MockDatabase::new();
    let repo = UserRepository::new(mock_db);

    let user = create_test_user();
    let result = repo.create(user).await;

    assert!(result.is_ok());
}
```

---

### 19.3 Integration Testing

**Scope**: API endpoints, database operations, external service integrations

**Tools**:
- HTTP testing: `actix-web::test`
- Database: MongoDB test database
- Test containers: Docker test containers

**Test Setup**:
```rust
#[actix_web::test]
async fn test_create_user_endpoint() {
    // Setup test database
    let db = setup_test_database().await;

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(create_user)
    ).await;

    // Make request
    let req = test::TestRequest::post()
        .uri("/api/v1/users")
        .set_json(&json!({
            "email": "test@example.com",
            "username": "testuser",
            "password": "SecureP@ssw0rd123",
            "first_name": "Test",
            "last_name": "User"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assertions
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["email"], "test@example.com");
    assert!(body["id"].is_string());

    // Cleanup
    cleanup_test_database(db).await;
}
```

**API Test Coverage**:

| Endpoint | Test Cases |
|----------|------------|
| POST /api/v1/users | Valid creation, duplicate email, weak password, invalid email |
| GET /api/v1/users/:id | Existing user, non-existent user, unauthorized access |
| PUT /api/v1/users/:id | Valid update, email change, unauthorized update |
| DELETE /api/v1/users/:id | Soft delete, hard delete, self-delete prevention |
| POST /api/v1/auth/login | Valid credentials, invalid credentials, account locked, MFA required |
| POST /api/v1/auth/refresh | Valid refresh token, expired token, revoked token |
| POST /api/v1/auth/logout | Valid logout, logout all sessions |
| POST /api/v1/roles/assign | Valid assignment, duplicate assignment, insufficient permissions |

---

### 19.4 End-to-End (E2E) Testing

**Scope**: Complete user workflows from UI to database

**Tools**:
- HTTP client: `reqwest` for Rust
- Test scripts: `tests/e2e/` directory
- Shell scripts: `tests/run_all_tests.sh`

**Critical Workflows**:

**1. User Registration & Verification**:
```bash
#!/bin/bash
# Test: Complete user registration flow

# Step 1: Register user
response=$(curl -X POST http://localhost:3003/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newuser@test.local",
    "username": "newuser",
    "password": "SecureP@ssw0rd123",
    "first_name": "New",
    "last_name": "User"
  }')

user_id=$(echo $response | jq -r '.id')
echo "✅ User created: $user_id"

# Step 2: Verify email (simulate clicking verification link)
# ... verification logic ...

# Step 3: Login with new credentials
login_response=$(curl -X POST http://localhost:3003/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email_or_username": "newuser@test.local",
    "password": "SecureP@ssw0rd123"
  }')

access_token=$(echo $login_response | jq -r '.access_token')
echo "✅ Login successful, token received"

# Step 4: Access protected resource
profile_response=$(curl -X GET http://localhost:3003/api/v1/users/me \
  -H "Authorization: Bearer $access_token")

echo "✅ Profile retrieved: $(echo $profile_response | jq -r '.email')"
```

**2. Password Reset Flow**:
```bash
# Step 1: Request password reset
curl -X POST http://localhost:3003/api/v1/auth/password-reset \
  -H "Content-Type: application/json" \
  -d '{"email": "user@test.local"}'

# Step 2: Extract reset token from email (simulated)
# reset_token=$(get_reset_token_from_email)

# Step 3: Complete password reset
curl -X POST http://localhost:3003/api/v1/auth/password-reset/complete \
  -H "Content-Type: application/json" \
  -d '{
    "token": "'$reset_token'",
    "new_password": "NewSecureP@ssw0rd456"
  }'

# Step 4: Login with new password
curl -X POST http://localhost:3003/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email_or_username": "user@test.local",
    "password": "NewSecureP@ssw0rd456"
  }'
```

**3. Role Assignment & Permission Check**:
```bash
# Step 1: Admin login
admin_token=$(login_as_admin)

# Step 2: Assign role to user
curl -X POST http://localhost:3003/api/v1/roles/assign \
  -H "Authorization: Bearer $admin_token" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "'$user_id'",
    "role_id": "editor"
  }'

# Step 3: Verify user has new permissions
curl -X GET http://localhost:3003/api/v1/users/$user_id/permissions \
  -H "Authorization: Bearer $admin_token"

# Step 4: Test user can access editor-only resource
user_token=$(login_as_user $user_id)
curl -X POST http://localhost:3003/api/v1/posts \
  -H "Authorization: Bearer $user_token" \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Post", "content": "..."}'
```

---

### 19.5 Security Testing

**Security Test Categories**:

**1. OWASP Top 10 Tests**:

| OWASP Risk | Test | Expected Behavior |
|------------|------|-------------------|
| A01:2021 Broken Access Control | Attempt to access other user's data | 403 Forbidden |
| A02:2021 Cryptographic Failures | Verify password hashing (Argon2id) | Hash length >= 60 chars |
| A03:2021 Injection | SQL/NoSQL injection in inputs | Sanitized, no execution |
| A04:2021 Insecure Design | Brute force password attempts | Account lockout after 5 attempts |
| A05:2021 Security Misconfiguration | Check security headers | HSTS, X-Frame-Options, etc. |
| A06:2021 Vulnerable Components | Dependency audit | No known vulnerabilities |
| A07:2021 Authentication Failures | Weak password attempt | Rejected with error |
| A08:2021 Data Integrity Failures | Tamper with JWT token | Rejected (signature invalid) |
| A09:2021 Logging Failures | Check audit logs | All actions logged |
| A10:2021 SSRF | External URL in profile picture | Validated, restricted domains |

**2. Password Security Tests**:
```rust
#[test]
fn test_passwords_are_hashed_with_argon2() {
    let password = "SecureP@ssw0rd123";
    let hash = hash_password(password);

    // Verify hash format
    assert!(hash.starts_with("$argon2id$"));

    // Verify hash length
    assert!(hash.len() >= 60);

    // Verify verification works
    assert!(verify_password(password, &hash));

    // Verify wrong password fails
    assert!(!verify_password("WrongPassword", &hash));
}

#[test]
fn test_weak_passwords_rejected() {
    let weak_passwords = vec![
        "short",
        "nouppercase123!",
        "NOLOWERCASE123!",
        "NoDigits!",
        "NoSpecial123",
        "password123",  // common password
    ];

    for weak in weak_passwords {
        assert!(validate_password_strength(weak).is_err());
    }
}
```

**3. JWT Security Tests**:
```rust
#[test]
fn test_jwt_token_expiry_enforced() {
    let expired_token = create_expired_jwt();
    let result = validate_jwt(expired_token);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Token expired");
}

#[test]
fn test_jwt_signature_validation() {
    let token = create_valid_jwt();
    let tampered_token = tamper_with_token(token);

    let result = validate_jwt(tampered_token);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid signature");
}

#[test]
fn test_revoked_tokens_rejected() {
    let token = create_valid_jwt();
    revoke_token(token.jti);

    let result = validate_jwt(token);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Token revoked");
}
```

**4. Account Lockout Tests**:
```rust
#[actix_web::test]
async fn test_account_lockout_after_failed_attempts() {
    let app = create_test_app().await;

    // Attempt 5 failed logins
    for _ in 0..5 {
        let req = test::TestRequest::post()
            .uri("/api/v1/auth/login")
            .set_json(&json!({
                "email_or_username": "user@test.local",
                "password": "WrongPassword"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    // 6th attempt should result in account lockout
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(&json!({
            "email_or_username": "user@test.local",
            "password": "CorrectPassword"  // Even with correct password
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 423);  // Locked

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error_code"], "AUTH_ACCOUNT_LOCKED");
}
```

---

### 19.6 Performance Testing

**Load Testing with `hey`** (HTTP load generator):

```bash
# Test: Authentication endpoint under load
hey -n 10000 -c 100 -m POST \
  -H "Content-Type: application/json" \
  -d '{"email_or_username":"test@test.local","password":"SecureP@ssw0rd123"}' \
  http://localhost:3003/api/v1/auth/login

# Expected results:
# - Response time (p99): <200ms
# - Success rate: >99%
# - Throughput: 500+ req/sec
```

**Database Query Performance**:
```rust
#[tokio::test]
async fn test_user_lookup_performance() {
    let db = setup_test_database_with_1m_users().await;

    let start = std::time::Instant::now();

    // Test email lookup (indexed)
    let result = db.users()
        .find_one(doc! { "email": "user_500000@test.local" }, None)
        .await;

    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(duration < std::time::Duration::from_millis(100));  // <100ms
}
```

**Concurrent User Simulation**:
```rust
#[tokio::test]
async fn test_concurrent_user_creation() {
    let app = create_test_app().await;
    let mut handles = vec![];

    // Simulate 100 concurrent user registrations
    for i in 0..100 {
        let app_clone = app.clone();
        let handle = tokio::spawn(async move {
            let req = test::TestRequest::post()
                .uri("/api/v1/users")
                .set_json(&json!({
                    "email": format!("concurrent_user_{}@test.local", i),
                    "username": format!("concurrent_{}", i),
                    "password": "SecureP@ssw0rd123",
                    "first_name": "Test",
                    "last_name": "User"
                }))
                .to_request();

            test::call_service(&app_clone, req).await
        });
        handles.push(handle);
    }

    // Wait for all to complete
    let results = futures::future::join_all(handles).await;

    // Verify all succeeded
    for result in results {
        let resp = result.unwrap();
        assert!(resp.status().is_success());
    }
}
```

---

### 19.7 Test Data Management

**Test User Factory**:
```rust
pub fn create_test_user(suffix: &str) -> User {
    User {
        id: Uuid::new_v4().to_string(),
        username: format!("testuser_{}", suffix),
        email: format!("test_{}@test.local", suffix),
        password_hash: hash_password("SecureP@ssw0rd123"),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        status: "active".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        last_login: None,
        phone_number: None,
        profile_picture_url: None,
        email_verified: true,
        deleted_at: None,
    }
}
```

**Test Cleanup**:
```bash
#!/bin/bash
# tests/test_cleanup.sh

# Remove all test users
mongosh --eval 'db.users.deleteMany({ email: /@test\.local$/ })'

# Remove test audit logs
mongosh --eval 'db.audit_logs.deleteMany({ user_id: { $in: [...] } })'

# Remove test sessions
mongosh --eval 'db.sessions.deleteMany({ user_id: { $in: [...] } })'

echo "✅ Test data cleaned up"
```

---

### 19.8 Continuous Integration (CI) Tests

**GitHub Actions Workflow**:
```yaml
name: Sapiens CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      mongodb:
        image: mongo:7.0
        ports:
          - 27017:27017
        env:
          MONGO_INITDB_ROOT_USERNAME: root
          MONGO_INITDB_ROOT_PASSWORD: password

      redis:
        image: redis:7-alpine
        ports:
          - 6379:6379

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true

      - name: Run unit tests
        run: cargo test --lib

      - name: Run integration tests
        run: cargo test --test '*'
        env:
          MONGODB_URI: mongodb://root:password@localhost:27017
          REDIS_URL: redis://localhost:6379

      - name: Run E2E tests
        run: ./tests/run_all_tests.sh

      - name: Check code coverage
        run: cargo tarpaulin --out Xml

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
```

---

## 20. Glossary & References

### 20.1 Glossary of Terms

| Term | Definition |
|------|------------|
| **RBAC** | Role-Based Access Control - Authorization model where permissions are assigned to roles, and roles are assigned to users |
| **MFA** | Multi-Factor Authentication - Security method requiring two or more verification factors |
| **JWT** | JSON Web Token - Compact, URL-safe token format for securely transmitting information between parties |
| **TOTP** | Time-based One-Time Password - Temporary password valid for short period (typically 30 seconds) |
| **OAuth 2.0** | Authorization framework enabling third-party applications to obtain limited access to user accounts |
| **Argon2id** | Modern password hashing algorithm resistant to GPU cracking attacks |
| **GDPR** | General Data Protection Regulation - EU regulation on data protection and privacy |
| **CCPA** | California Consumer Privacy Act - California state privacy law |
| **OWASP** | Open Web Application Security Project - Nonprofit foundation focused on software security |
| **DSAR** | Data Subject Access Request - GDPR right for individuals to access their personal data |
| **UUID** | Universally Unique Identifier - 128-bit identifier guaranteed to be unique |
| **E.164** | International standard for telephone number format |
| **RFC 5322** | Internet Message Format - Standard defining email address format |
| **TTL** | Time To Live - Mechanism to limit lifespan of data in a system |
| **CSRF** | Cross-Site Request Forgery - Attack that forces authenticated users to submit unwanted requests |
| **XSS** | Cross-Site Scripting - Security vulnerability allowing attackers to inject malicious scripts |
| **SQL Injection** | Attack technique inserting malicious SQL code into queries |
| **DDD** | Domain-Driven Design - Software design approach focusing on core domain and domain logic |
| **Aggregate Root** | Primary entity in an aggregate that other entities reference |
| **Value Object** | Immutable object defined by attributes rather than identity |
| **Repository Pattern** | Abstraction layer between domain and data access |
| **gRPC** | Google Remote Procedure Call - High-performance RPC framework |
| **Protocol Buffers** | Google's language-neutral data serialization format |
| **Microservices** | Architectural style structuring application as collection of loosely coupled services |
| **Monorepo** | Single repository containing multiple projects/services |

### 20.2 Acronyms & Abbreviations

| Acronym | Full Form |
|---------|-----------|
| API | Application Programming Interface |
| BRD | Business Requirements Document |
| CDN | Content Delivery Network |
| CLI | Command Line Interface |
| CRUD | Create, Read, Update, Delete |
| DTO | Data Transfer Object |
| FR | Functional Requirement |
| FTE | Full-Time Equivalent |
| HTTP | Hypertext Transfer Protocol |
| HTTPS | HTTP Secure |
| IP | Internet Protocol |
| JSON | JavaScript Object Notation |
| KPI | Key Performance Indicator |
| MVP | Minimum Viable Product |
| NFR | Non-Functional Requirement |
| NPS | Net Promoter Score |
| ORM | Object-Relational Mapping |
| PII | Personally Identifiable Information |
| QA | Quality Assurance |
| REST | Representational State Transfer |
| ROI | Return on Investment |
| RTO | Recovery Time Objective |
| RPO | Recovery Point Objective |
| S3 | Amazon Simple Storage Service |
| SDK | Software Development Kit |
| SIEM | Security Information and Event Management |
| SLA | Service Level Agreement |
| SMS | Short Message Service |
| SOC 2 | Service Organization Control 2 |
| SQL | Structured Query Language |
| SSO | Single Sign-On |
| TLS | Transport Layer Security |
| UAT | User Acceptance Testing |
| UI | User Interface |
| UMS | User Management System |
| URL | Uniform Resource Locator |
| UX | User Experience |
| VPS | Virtual Private Server |
| WCAG | Web Content Accessibility Guidelines |

### 20.3 References

**Standards & Specifications**:
- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)
- [NIST SP 800-63B Digital Identity Guidelines](https://pages.nist.gov/800-63-3/sp800-63b.html)
- [RFC 5322 Internet Message Format](https://tools.ietf.org/html/rfc5322)
- [RFC 7519 JSON Web Token (JWT)](https://tools.ietf.org/html/rfc7519)
- [RFC 6238 TOTP: Time-Based One-Time Password](https://tools.ietf.org/html/rfc6238)
- [OAuth 2.0 RFC 6749](https://tools.ietf.org/html/rfc6749)
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)

**Compliance & Regulations**:
- [GDPR Official Text](https://gdpr-info.eu/)
- [CCPA Full Text](https://oag.ca.gov/privacy/ccpa)
- [OWASP Top 10 2021](https://owasp.org/Top10/)

**Technical Documentation**:
- [Argon2 Specification](https://github.com/P-H-C/phc-winner-argon2)
- [MongoDB Documentation](https://docs.mongodb.com/)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Actix-Web Documentation](https://actix.rs/docs/)
- [Protocol Buffers Documentation](https://developers.google.com/protocol-buffers)

**Security Resources**:
- [IBM Cost of a Data Breach Report 2025](https://www.ibm.com/security/data-breach)
- [SANS Password Policy Guidelines](https://www.sans.org/security-resources/policies/general/pdf/password-policy)
- [CWE Top 25 Most Dangerous Software Weaknesses](https://cwe.mitre.org/top25/)

**Industry Benchmarks**:
- [Auth0 State of Secure Identity Report](https://auth0.com/resources/whitepapers/state-of-secure-identity-report)
- [Okta Businesses @ Work Report](https://www.okta.com/businesses-at-work/)

---

## Document Approval

**Prepared By**:
- Name: Farid Hidayat
- Title: CEO, StartApp
- Date: November 19, 2025
- Signature: ___________________

**Reviewed By**:
- Name: [Technical Lead]
- Title: Lead Software Architect
- Date: ___________________
- Signature: ___________________

**Approved By**:
- Name: [Stakeholder]
- Title: Product Owner
- Date: ___________________
- Signature: ___________________

---

## Document Change Log

All changes to this document must be logged below:

| Version | Date | Section | Change Description | Changed By |
|---------|------|---------|-------------------|------------|
| 2.0 | 2025-11-19 | All | Comprehensive expansion with detailed specifications from domain.md | Farid Hidayat |
| 2.1 | TBD | TBD | TBD | TBD |

---

**End of Business Requirements Document**

---

*This document is confidential and proprietary to StartApp. Unauthorized distribution is prohibited.*

*For questions or clarifications, please contact: Farid Hidayat (farid@startapp.id)*

