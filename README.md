# Vertretungsplan

 ## Design

  ### Fonts
   - Heading:   Poppins     SemiBold    40px
   - Body:      Rubik       Regular     16px
  ### Colors (not final)
   ```
   --text: #000019;
   --background: #ffffff;
   --primary: #003a6c;
   --secondary: #e2edf6;
   --accent: #1c254a;
   ```

 ## ToDo's

  ### UI Design

   - [x] simple design for main page
   - [x] design Header
   - [x] colorsheme with [https://www.realtimecolors.com/](https://www.realtimecolors.com/)
   - [x] choose fonts
   - [x] maybe Heute/Morgen button beneath heading
   - [x] Header with Login Profile and Settings
   - [ ] new color palette
   - [ ] Impressum
   - [ ] design Background
   - [ ] Login page
   - [ ] Settings Page
   - [ ] Admin Panel
   - [ ] Aushänge
 

   #### Personal

   - [x] Heute/Morgen vllt. neben Content statt drüber
   - [x] Icons on the left side of the content line
   - [x] Stift symbol unten rechts onclick: Text daneben mit "Studenplan bearbeiten"
   - [ ] Choose Icons for EVA, Raumänderung, Vertretung, etc.

   #### Dein Stundenplan

   - [x] Stundenplan wird normal angezeigt
   - [x] unten rechts Stift um in Bearbeitungsmodus zu wechseln
   - [x] in Bearbeitungsmodus bei jedem Stundenslot ein Plus um Hinzufügen overlay zu öffnen
   - [x] Hinzufügen overlay (Fach, Lehrer, Kürzel, Raum)

  ### Frontend (svelte)

   - [ ] css: color variables
   - [ ] implement VP design
   - [ ] implement Personal design
   - [ ] implement Stundenplan design

   #### Dein Stundenplan
   - [ ] normale Anzeige: wird nur so hoch, wie der längste Tag
   - [ ] Bearbeitungsmodus: volle 10 Stunden

  ### Backend (rust)
