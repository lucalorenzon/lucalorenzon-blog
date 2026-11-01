## Summary

I'm a product engineer who works across implementation, architecture and collaboration. I've delivered complex solutions in e-commerce, enterprise integration and order fulfillment through hands-on work and cross-functional problem-solving. Continuous learner focused on software languages (Rust, TypeScript, Python, Java), security practices, infrastructure automation, Domain-Driven Design and socio-technical systems thinking.

## Experience

### Enterprise Architecture Manager

*Italian Luxury Brand* · February 2024 – present

Contributing to the architecture strategy and to the governance of the application portfolio, together with the group's architecture functions; scouting technologies and vendors; acting as the link between business and technical stakeholders, and mentoring teams on architectural principles.

#### Main work

- **Payment provider migration**\
  Architectural support for moving payments from one provider to another: impact review on order management, reconciliation and anti-fraud reporting. Wrote two Terraform providers in Go, so that the configuration of third-party platforms is versioned as code, and a command-line tool in Rust for a payment platform's management API.
- **First Rust service in production**\
  Rewrote a small production microservice in Rust, replacing the Java version I had written years before: about 5 MB of resident memory, nearly constant, against a 250 MB baseline, in a container built `FROM scratch`, with no operating system inside. Traffic is low, but the same the Java version handled.
- **Demand approval and AI project governance**\
  Architectural contributor to demand evaluation and tech gate reviews, and architecture governance in the technical reviews and approvals of AI projects.
- **Architecture assessments**\
  Assessment of existing platforms, with alternative architectures proposed.
- **Product information enrichment**\
  Analysis of the process that enriches product information, from a product's conception to its going on sale.
- **Request for proposal for DevOps services**\
  Definition of the technical details and of the architectural part of the request for proposal.
- **Load testing**\
  Load tests with k6 on back-office systems and internal tools, including distributed load generation on Kubernetes, set up and acceptance-tested with the DevOps team. Generating the load is the start: then comes the analysis of the observability data collected and the search for the bottlenecks, sometimes entirely on my own, sometimes leading with others' support, sometimes in support of others.
- **Contributions to DevOps**\
  Integrated deployment pipelines with a secrets manager, and built a proof of concept of dynamic, short-lived database credentials used through an application's connection pool.

**Skills:** Enterprise Architecture · Architecture Strategy · Portfolio Architecture Participation · Technology Scouting · Cross-organizational Facilitation · Technical Advisory · Stakeholder Management · Infrastructure Strategy · Organizational Alignment

### Senior Software Engineer

*Italian Luxury Brand* · September 2020 – February 2024

Working in cross-functional agile teams (LeSS framework) on the e-commerce platform's move from a monolith to microservices, product discoverability replatforming included, through API design, Infrastructure-as-Code (Terraform) and distributed systems practices.

#### Main work

- **New market in APAC (Singapore)** · May 2021 – December 2021\
  Incremental work with cross-functional agile teams on every activity and integration around the e-commerce system needed to open the new site: stock, prices, assortment, stores, warehouse, copy, categorization, navigation, SEO, payment, invoicing.
- **New catalog API** · September 2020 – May 2021\
  Incremental design and implementation, in a cross-functional agile team, of a new version of the catalog API that exposes the brand's catalog to its partners.
- **Process improvement with Domain-Driven Design**\
  Identified and engaged the key stakeholders, ran interviews and took part in collaborative modeling sessions (domain storytelling, user story mapping) to analyze and improve business processes across the digital platform.
- **Communities of Practice**\
  E-commerce advisor in the Architecture and Infrastructure Communities of Practice: technical guidance on solutions and patterns, Infrastructure-as-Code best practices, knowledge exchange across teams during the organizational transformation.
- **Performance reviews of the e-commerce site** · September 2020 – February 2024\
  No longer the owner, but still called on to lead the reviews and taking an active part in their development and execution, with k6 on the site's microservices such as the catalog API. Did the analysis of the observability data and the search for the bottlenecks, making sure it was part of every review.

**Skills:** Domain-Driven Design · Microservices Architecture · Infrastructure-as-Code (Terraform) · Process Optimization · LeSS Framework · Rust · E-commerce Subject Matter Expert · Community of Practice · Stakeholder Engagement

### eBusiness SAP Hybris Solution Architect

*Italian Luxury Brand* · November 2016 – September 2020

Solution architect for the brand's e-commerce on SAP Hybris, coordinating development, infrastructure and business teams, internal and external. The Hybris upgrade and the data center migration brought the response time from about 545 ms to 30 ms.

#### Main work

- **Performance reviews of the e-commerce site** · November 2016 – September 2020\
  Owner of the periodic reviews: stress tests with Gatling, thread dumps and memory dumps; analysis of the results, identification of KPIs and dashboards, root cause analysis, possible solutions.
- **Integration of a new order management system** · July 2018 – September 2020\
  Solution architect in a two-year initiative with parent group stakeholders, external partners and cross-departmental teams: end-to-end order flows across e-commerce, warehouse, shipping and returns, and integrations for inventory supply types and stock allocation, prices, product information, payments, the financial system, cross-border invoicing, CRM, transactional emails, the sales assistance service, monitoring and alerting.
- **Data center migration** · July 2018 – October 2018\
  Move of the data center provider to another physical location: every production and non-production machine reinstalled with no interruption of service, and the e-commerce machines partly moved to the cloud.
- **Hybris version upgrade** · August 2017 – February 2018\
  Reviewed the partner's plan to reduce the impact on ongoing projects; led the technical decisions during implementation; assigned the activities among several vendors during QA and UAT; facilitated the knowledge transfer on the new platform to all vendors; led the performance tests and the improvements they suggested; designed the go-live plan and its follow-up activities, deploying heavy changes with little room for a courtesy page.
- **Automated deployments**\
  With a lean DevOps team, moved from manual processes to CI/CD and near-zero-downtime deployments, keeping the platform stable.
- **Technical review of business requirements** · February 2017 – September 2020\
  Reviewed the backend requirements in the business stories, assessed their feasibility and drawbacks, identified working solutions, and followed the partners' implementation.
- **From a single vendor to several** · November 2016 – February 2017\
  Took over the release management from the previous partner, and designed the release management guidelines for several vendors working together.

**Skills:** SAP Hybris · Solution Architecture · Order Management Systems · Enterprise Integration · Order Fulfillment Domain Expert · Infrastructure Collaboration · CI/CD · Performance Optimization · Cross-organizational Stakeholder Management

### E-commerce Solution Architect

*NTT DATA SpA* · June 2008 – November 2016

In the core technical team, led end-to-end e-commerce delivery (requirements, architecture, implementation, testing, maintenance) on SAP Hybris, ElasticPath and Konakart, for B2C, B2B and omnichannel clients in the Italian, UK and French markets. Technical reference across application and infrastructure, CI/CD pipelines included; specialized in search (Solr), payment integrations, pricing and promotion engines, multi-warehouse solutions and performance tuning (JMeter, JFR, GC analysis).

#### Main work

- **Pick & pay for an Italian supermarket chain** · March 2016 – October 2016\
  Migration of a pick & pay e-commerce from KonaKart to SAP Hybris (Solr, web services, Drools).
- **B2B e-commerce for an international security products manufacturer** · March 2015 – March 2016\
  Analysis, design and development of a B2B platform on SAP Hybris, with SAP ERP ECC integrated through SAP Hybris Data Hub.
- **Worldwide e-commerce for an Italian home appliance multinational** · December 2011 – March 2015\
  Analysis, design and development of the worldwide e-commerce on SAP Hybris.
- **Worldwide e-commerce for small appliances of the same multinational** · June 2013 – October 2013\
  E-commerce site selling one of its brands' small domestic appliances.
- **Online payments for an Italian airport operator** · January 2011\
  Integration of the operator's new online platform with the online payment gateway (Java).
- **E-commerce and corporate website for an Italian electronics retailer** · July 2008 – December 2011\
  Development of the product import interface and ongoing evolution of the e-commerce site.
- **Commercial bids**\
  Technical assessments and solution proposals for major brands in luxury fashion, telecommunications and retail across Europe, several of them won.

**Skills:** SAP Hybris · E-commerce Architecture · ElasticPath · Konakart · Java · Spring Framework · Solr · Payment Integration · JMeter · Performance Optimization · CI/CD · Solution Architecture

### TIBCO Senior Consultant

*NTT DATA SpA* · April 2006 – June 2008

TIBCO integration and workflow consulting for telecommunications enterprise systems, supporting Telecom Italia's CRM-connected systems.

**Skills:** Telecommunications · Integration · Business Analysis · Java · TIBCO

### Senior Java Developer & TIBCO Support Consultant

*Objectway SpA* · January 2001 – April 2006

Progressed through multiple roles delivering Java enterprise applications and TIBCO integration solutions for telecommunications clients.

- Member of the official TIBCO support team for Telecom Italia's order management system design and implementation.
- Developed a workflow orchestration engine for telecommunications number portability processes.
- Built an assembly-line tracking system for electronics manufacturing.
- Created business process monitoring and invoice management applications.

**Skills:** Integration · Java · TIBCO · CORBA

### C++ Developer

*Pegaso Srl* · September 2000 – December 2000

C++ development for telecommunications systems integration (TIBCO Rendezvous).

## Technical expertise and continuous learning

- **Rust:** advocating its adoption in the company, with a first service in production as a concrete demonstration; more than three years of self-directed study and personal projects in systems programming, concurrency and infrastructure tooling.
- **Cybersecurity:** active practitioner on the TryHackMe platform ([public profile](https://tryhackme.com/p/Lvk73r)), with a focus on offensive security techniques.
- **Architecture and design:** training in Lean Architecture (James Coplien), Hexagonal Architecture (Alistair Cockburn) and Domain-Driven Design.
- **Machine learning:** TensorFlow development, Machine Learning Operations (MLOps) and Generative Adversarial Networks (DeepLearning.AI).

## Core competencies

- **Architecture:** Enterprise Architecture, Solution Architecture, Domain-Driven Design, Hexagonal Architecture, Data Context Interaction, CQRS, Event Sourcing, Microservices, API Design
- **Collaboration:** Cross-functional Collaboration, Stakeholder Management, Community of Practice
- **Platforms and systems:** E-commerce Platforms, Order Management Systems, System Integration, Supply Chain & Logistics, SAP Hybris, Manhattan Active Omni, Payment Platforms (Adyen, Cybersource, PayPal, Setefi, Realex Payments, Affirm, 4gift), TIBCO, CORBA
- **Programming:** Rust, Python, TypeScript, Java
- **Infrastructure:** Infrastructure-as-Code, CI/CD, Distributed Systems, Performance Optimization, Secrets Management (HashiCorp Vault)
- **Search and data:** Solr, Algolia, Elasticsearch, OramaSearch
- **Testing and optimization:** k6, Gatling, JMeter, profiling and memory dump analysis
- **Methodologies:** Agile, LeSS Framework

## Education

### Master's degree in Software Engineering

*Sapienza Università di Roma* · 1992 – 2000

Thesis: CASPER to ALsp compiler, verifying security protocols as planning in logic programming.

## Certifications and training

- LeSS Framework Training (Bas Vodde)
- Lean Architecture Workshop (James Coplien)
- Hexagonal Architecture Workshop (Alistair Cockburn)
- Software Craftsmanship Conference (SoCraTes)
- Machine Learning Operations (DeepLearning.AI)
- Offensive Security Training (TryHackMe)

## Languages

Italian (native), English (fluent)
