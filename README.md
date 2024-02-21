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
   - [x] Login page
   - [ ] background for Login page
   - [ ] Settings Page
   - [ ] Admin Panel
   - [ ] Aushänge
 

   #### Personal

     - [ ] Choose Icons for EVA, Raumänderung, Vertretung, etc.

   #### Dein Stundenplan

   
  ### Frontend (svelte)

   - [ ] improve all gaps!
   - [ ] implement Personal design
   - [ ] implement Stundenplan design
   - [ ] click cursor on buttons
   - [ ] button animations
   - [ ] use browsers back button (url hash)

   - [ ] normal plan: get_rows with POST request to /get_rows
   - [ ] put all ids of changed rows into changed store

   #### Dein Stundenplan
  
  ### Backend (rust)

   
   - [ ] delete row function

   - [ ] overwrite exsiting row in changed if same id
