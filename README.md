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

   - [ ] day logic

   - [ ] admin: show last db entry and not current_day

   - [ ] sort options (class, fach, etc.)
   - [ ] show update time

   - [ ] admin: day selection
   - [ ] admin: if day doesnt exists show button "add day" instead of automatically adding it

   
  
  ### Backend (rust)

   
   - [x] delete row function

   - [ ] login backend

   - [ ] admin request only for admin account

   - [ ] return Errors insted of .expect()

   - [ ] german week days

   - [x] database row creation from normal user?

   - [x] implement get_next_day


