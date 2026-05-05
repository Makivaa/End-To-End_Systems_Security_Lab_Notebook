# Reimagining Secuirty for the Agentic AI
This page is to be on a talk given by Neta Haiby & Raji Vanninathan at UCSC 5/4/26

## Why agents change the rules

### Anatomy of an agent
- Plan
- Remember
- Act
- Access

Agents are probabilistic and thus cannot be predicted on what they're going to do. Permissions may change on a per-task basis.
Even a supposedly simple task such as planning a vacation gives an agent access to many systems, all of which can go wrong. (i.e. Unauthorized purchases, budget overrun, privacy breach, give untrusted third-party access to sensitive information)

## The new threat surface

### AI Usage
- Sensitive infomation discolsure
- Shadow IT/harmful LLM app/plugin

### Ai Agents
- Intent Breaking
- Tools misuse
- Prompt Injection
- Context Poisoning
- Data leak
- Insecuire MCP servers

### AI Platform
- Training Data Poisoning
- Model Vulnerabilities & Theft

## A mental model for securing agents

### Identity - Every agent is named & governed
Every agent should have a distinct identity that can act on behalf of people with limits.

### Access - Least privilege, scoped to task
Access accumulates over time, combination of these different privileges lead to issues.
- Just enough - only required access
- Just in time - human must give access
- Step up

### Observe - See what agents do in real time
- Real time activity
- Continuous monitoring
- Register every agent

### Secure - Protect data, systems, & actions
- Sanitize input
- Prompt isolation
- Output validation
- Limit the impact - isolate agents
- Control every action - enfore policies in real time
- Recover fast - Detect issues early, contain quickly, restore safely

### Oversight - Humans in the loop
- Auto - read only, low risk (Log)
- Notify - routine writes, internal (Tell user, allow undo)
- Approve - sensitive: external eamil, money, etc (Block until user says yes)
- Deny - Irreversible or out of policy (Hard stop)

## The future

### Multi-agent systems
- Trust transitivity - A trusts B and B trusts C can A trust C?
- Permission propagation - Whose scope wins when agents delegate
- Collective behavior - How to deal with swarm of agents

### Remember
Agetns are autonomous
New threats need new defenses
Five pillars

# Security Vulnerability Research - Be the one to notice when AI breaks
This page is to be on a talk given by Neta Haiby & Raji Vanninathan at UCSC 5/4/26

## Why vulnerability research is hidden
Heartbleed - A bug in OpenSSL's hearbeat extension that allowed a server to echo back up to 64KB of its own memory

Understanding -> Breaking -> Proving
Reverse engineer how the target works, then find ways to make the target do something it'snot inteded to do, finally turn that knowledge into a reliable proof-of-concept exploit


## How the field evolved
Manual Era -> Tool-Assisted -> Scale & Platform -> AI-Augmented

## Where AI fits
### What AI Does Well
- Scan code instantly
- Spot known vulnerability patterns
- Generate fuzz inputs at scale
- Triage adn classify bug reports

### Where Humans Still Win
- Novel attack surface 
- Binary exploitating & RE
- Chaining vulnerabilities
"A vulnerability is just a celebrity bug"

## The AI safety frontier
### New Dimensions
- Natural language
- Non-Determinism
- AI acts on our behalf
- Agents have identity and tasks are dynamic
- Barrier to entry is low