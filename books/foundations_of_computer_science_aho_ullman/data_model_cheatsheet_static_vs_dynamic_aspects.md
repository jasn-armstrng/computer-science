### **Data Model Cheatsheet: Identifying Static vs. Dynamic Aspects**

When analyzing a **data model**, you can break it down into **static** and **dynamic** aspects. Use this **cheat sheet** to quickly classify components in any system.

---

## **1. Static Aspects (Type System)**
**Definition**:
The **static** part defines what objects exist in the system, their properties, and the constraints on their values. These aspects do **not** change in structure but can have instances that change dynamically.

### **Key Characteristics**:
✔ **Defines "what exists" in the model**
✔ **Fixed properties or rules** (unchanging structure)
✔ **Predefined categories, types, or classifications**
✔ **Stored data that does not inherently change behavior**

### **Examples**:
🔹 **Game World / Environment** (e.g., maps, levels, predefined obstacles)
🔹 **Entities & Objects** (e.g., characters, power-ups, items)
🔹 **Rules & Constraints** (e.g., physics laws, game mechanics)
🔹 **Types & Attributes** (e.g., player stats, item classifications)

💡 **Shortcut Tip**:
If you can define it **before** the game or system starts running and it stays consistent across executions, it's **static**.

---

## **2. Dynamic Aspects (Operations on Data)**
**Definition**:
The **dynamic** part describes **how data changes over time**—the operations, behaviors, and interactions between objects.

### **Key Characteristics**:
✔ **Defines "what happens" in the model**
✔ **State changes over time** (values update, events occur)
✔ **Operations on objects (functions, transformations, interactions)**
✔ **Player or system-driven updates**

### **Examples**:
🔹 **Character Movement & Interactions** (e.g., Pac-Man eating pellets, ghost AI changing behavior)
🔹 **State Transitions** (e.g., player losing a life, power-ups activating)
🔹 **Event-Driven Changes** (e.g., scoring, time-based enemy behaviors)
🔹 **Collisions & Physics** (e.g., objects bouncing, getting destroyed)
🔹 **Game Progression** (e.g., leveling up, difficulty increasing)

💡 **Shortcut Tip**:
If it involves **time-dependent updates** or **modifications of existing objects**, it's **dynamic**.

---

### **Quick Comparison Table**

| **Aspect**  | **Static (Type System)** | **Dynamic (Operations on Data)** |
|------------|------------------------|--------------------------------|
| **Definition** | Defines objects, properties, and constraints | Defines changes, interactions, and behaviors |
| **Does it change?** | No (fixed structure) | Yes (updates over time) |
| **Examples** | Game board, item types, character classes | Movement, AI behaviors, item collection |
| **Analogy** | A **blueprint** of a building | The **events and activities** happening inside the building |

---

### **Practical Steps to Identify Static vs. Dynamic Aspects**
1️⃣ **List all objects in the system** → Ask: _Do they exist independently of time?_
   - If YES → **Static**
   - If NO (change happens over time) → **Dynamic**

2️⃣ **Identify properties of objects** → Ask: _Do these properties remain the same throughout the game?_
   - If YES → **Static**
   - If NO (values change as the game runs) → **Dynamic**

3️⃣ **Analyze interactions** → Ask: _Does this involve movement, transformation, or state changes?_
   - If YES → **Dynamic**
   - If NO (it just defines structure) → **Static**

---

### **Example Breakdown: Space Invaders**
- **Static Aspects**:
  - The game screen layout (aliens, ship, barriers)
  - The types of entities (player, alien, bullet)
  - The scoring system

- **Dynamic Aspects**:
  - Alien movement pattern
  - Player shooting bullets
  - Aliens reacting to bullets (getting destroyed)
  - Score increasing upon hitting an alien

---

### **Final Tip: Think Like a Programmer**
🔹 **Static aspects** resemble **data structures** (objects, types, schemas).
🔹 **Dynamic aspects** resemble **functions and algorithms** (state updates, AI, interactions).
