<script>
  import PersonalRow from "./PersonalRow.svelte";
</script>


<section class="whole-site">

  <div class="personal-container">
    <h2 class="personal-heading">Wichtige Informationen</h2>

    <div class="personal-grid">
      <div class="date">
        <p class="day">Mo</p>
        <p class="day-num">08</p>
      </div>
      <div></div>
      <div class="personal-info">
        <PersonalRow text={"Englisch in der 3. Stunde entfällt!"}/>
        <PersonalRow text={"Deutsch bei Frau Bindewald entfällt in der 5. Stunde!"}/>
        <PersonalRow text={"Englisch in der 3. Stunde entfällt!"}/>
      </div>

      

      <div class="date">
        <p class="day">Di</p>
        <p class="day-num">09</p>
      </div>
      <div></div>
      <div class="personal-info">
        <PersonalRow text={"Englisch in der 3. Stunde entfällt!"}/>
        <PersonalRow text={"Deutsch bei Frau Bindewald entfällt in der 5. Stunde!"}/>
      </div>

    </div>
  </div>

</section>


<style>
  .whole-site {

    height: 91.5vh;
  }

  .personal-container {

    width: 50%;


    background-color: var(--secondary);

    border-style: none;
    border-radius: 29px;


    position: relative;

    left: 50%;
    top: 50%;

    transform: translate(-50%, -50%);


    display: flex;

    flex-direction: column;

    align-items: center;

  }


  .personal-heading {

    font-family: 'Poppins', sans-serif;

    font-size: 40px;

    color: var(--primary);

  }

  .personal-grid {

    width: 100%;

    display: grid;

    grid-template-columns: 15% 4% 81%;

    row-gap: 60px;

    margin-bottom: 10vh;

  }

  .personal-info {

    margin-top: 9px;

  }

  .date {

    width: 100%;

    text-align: center;
  }

  .personal-info {

    width: 100%;

  }

  .day {

    margin: 0px 0px 0px 0px;

    font-family: 'Poppins', sans-serif;

    font-size: 30px;

    color: var(--accent);

  }
  .day-num {

    margin: 0px 0px 0px 0px;
  
    
    font-family: 'Poppins', sans-serif;

    font-size: 16px;

    line-height: 8px;
    
    color: var(--accent);
  }
</style>
