<script>
  import Info from "./Info.svelte";
  import Table from "./Table.svelte";
  import { onMount } from "svelte";

  let current_day = {id: 1, date: "08.09.2023", week_day: "Montag"};

  let today_is_active = true;


  onMount(async () => {

    await get_current_day();
  });


  async function get_current_day() {


    today_is_active = true;




    const res = await fetch('/get_current_day_string', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        },
    })

    
    let json_res = await res.json();
  

    current_day.date = json_res.datum;
    current_day.week_day = json_res.week_day;




    const res2 = await fetch('/get_current_day', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        },
    })

    
    if (res2.status != 200) {

      console.log("no row")

      current_day.id = -1;

      return

    }

    let json_res2 = await res2.json();

    current_day.id = json_res2.id;

  }

  async function next_day() {

    today_is_active = false;


   const res = await fetch('/get_next_day_string', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        },
    })

    
    let json_res = await res.json();
  

    current_day.date = json_res.datum;
    current_day.week_day = json_res.week_day;




    const res2 = await fetch('/get_next_day', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        },
    })

    
    if (res2.status != 200) {

      console.log("no row")

      current_day.id = -1;

      return

    }

    let json_res2 = await res2.json();

    current_day.id = json_res2.id;

  }
</script>


<section>
  <div class="plan-container">
    <h2 class="plan-heading">{current_day.week_day} der {current_day.date}</h2>

    <div class="button-container">

      {#if today_is_active}
        <button class="active" on:click={get_current_day}>Heute</button>
        <button class="not-active" on:click={next_day}>Morgen</button>

      {:else}
        <button class="not-active" on:click={get_current_day}>Heute</button>
        <button class="active" on:click={next_day}>Morgen</button>

      {/if}

          </div>

    <div class="info-container">
      <h3 class="info-head">Infos</h3>

      <div class="infos">
        {#each Array(4) as _, i}
          <Info text={"Das ist eine wichtige Info!"} />
        {/each}
      </div>

    </div>

    <Table day_id={current_day.id} />


  </div>
</section>


<style>
  .plan-container {


    margin-top: 10vh;
    margin-bottom: 20vh;


    background-color: #e2edf6;

    border-style: none;
    border-radius: 29px;

    width: 78.5%;

    position: relative;

    left: 50%;

    transform: translate(-50%);


    display: flex;

    flex-direction: column;

    align-items: center;

  }

  .plan-heading {

    color: var(--primary);

    font-family: 'Poppins', sans-serif;

    font-weight: 600;

    font-size: 40px;

    margin: 6vh 0vh 5vh 0vh;

  }


  .active {

    padding-top: 8px;
    padding-bottom: 8px;

    margin-right: 5px;

    width: 110px;

    background-color: var(--accent);
  

    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: var(--secondary);

    border: none;
    border-radius: 10px;


     cursor: pointer;

  }

  .not-active {

    padding-top: 7px;
    padding-bottom: 7px;

    margin-left: 5px;

    background-color: var(--secondary);

    width: 110px;
    
    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: var(--accent);

    border: solid;
    border-color: var(--accent);
    border-width: thin;
    border-radius: 10px;

    
    cursor: pointer;
  }

  .not-active:hover {

    opacity: 70%;

  }

  .info-container {

    background-color: white;
  

    width: 60%;


    margin-top: 9vh;

    padding: 4vh 0vh;

    
    border: none;
    border-radius: 10px;

    box-shadow: 0px 6px 6px 4px rgba(0, 0, 0, 0.2);

  }

  .info-head {

    color: #000000;

    font-family: 'Poppins', sans-serif;

    font-weight: 600;

    font-size: 32px;

    margin: 0vh 0vh 0vh 6vh;


  }

  .infos {
    
    margin-top: 2vh

  }
</style>
