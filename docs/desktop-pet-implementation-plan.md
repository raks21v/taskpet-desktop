# TaskPets Windows Desktop Pet Implementation Plan

## Repository inspection result

The repository currently starts almost empty: it only contains `.gitkeep` plus Git metadata. There is no existing React dashboard, backend, or desktop application code to migrate, so Phase 1 creates the first real desktop-app foundation instead of extending a website.

## Product direction

TaskPets should be a Windows desktop pet application. Pets should live as small transparent, borderless, always-on-top desktop windows rather than inside a web dashboard. Each pet represents a daily-life domain:

- ApplyBee: jobs, internships, and applications.
- Codey: coding and projects.
- Sprout: learning and courses.
- Pixel: creativity and design.
- Droplet: hydration and water tracking.
- Fitbitty: fitness, walking, stretching, workouts, and step goals.

## Chosen technology stack

Phase 1 uses Tauri with Rust for the native desktop shell and a very small HTML/CSS pet surface for rendering the first pixel pet. This is not a React/browser dashboard: the app launches as a Windows desktop application, owns native windows, can create transparent borderless always-on-top pet windows, and can expose a system tray menu.

Tauri is a good fit because it supports:

- Transparent windows.
- Borderless windows.
- Always-on-top windows.
- Skipping the taskbar for pet windows.
- Native system tray menus.
- Rust-side persistence and filesystem integration for later XP/task storage.
- Multiple independent windows, which maps to one desktop pet per window.

## Phase 1 scope

Phase 1 intentionally implements only one simple pet so the desktop-window behavior can be proven before adding the full task game loop.

Included in Phase 1:

- Create a native TaskPets desktop app shell.
- Open one ApplyBee pet in its own small window.
- Make the ApplyBee window transparent, borderless, always on top, fixed size, and hidden from the taskbar.
- Render a cute animated CSS pixel-art ApplyBee with no visible rectangular background.
- Allow the window to be dragged by the pet stage.
- Show a tiny floating speech bubble when the pet is clicked.
- Add a system tray menu to show/hide ApplyBee or quit TaskPets.

Not included yet:

- Task creation and completion.
- XP gain and persistence.
- Levels and visual evolution.
- Multiple pet windows.
- Native asset pipeline for final pixel sprites.
- Hydration and fitness tracking.

## Future phases

### Phase 2: Task bubble and persistence

- Replace the placeholder speech bubble with a floating task panel.
- Add today's quest for ApplyBee.
- Complete the quest and award XP.
- Persist XP and task state locally, likely using a Rust-managed app data JSON or SQLite store.

### Phase 3: Multi-pet window manager

- Add Codey, Sprout, Pixel, Droplet, and Fitbitty as separate transparent windows.
- Add tray controls for showing/hiding each pet individually and showing/hiding all pets.
- Persist each pet's last desktop position.

### Phase 4: Animation and evolution

- Replace CSS prototype art with pixel sprite sheets.
- Add idle, bounce, walk, celebrate, sleep, and level-up animations.
- Add visual evolution stages tied to level thresholds.

### Phase 5: Domain-specific quests

- ApplyBee: applications, networking, resume updates, interview prep.
- Codey: coding sessions, bug fixes, project milestones.
- Sprout: lessons, readings, courses, practice.
- Pixel: sketches, design tasks, creative streaks.
- Droplet: daily water goal, quick glass/bottle logging, hydration progress XP.
- Fitbitty: step goals, walks, stretching, workouts, fitness quest XP.
