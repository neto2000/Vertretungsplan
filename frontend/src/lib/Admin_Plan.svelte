<script>
  import AdminTabelle from "./Admin_Tabelle.svelte";


  import { admin_rows } from "../stores";
  import { onMount } from "svelte";

  let info_text = "";

  let current_day = {id: 1, date: "17.01.2024", week_day: "Montag"};




  let rows = [];

  admin_rows.subscribe((value) => {

      rows = value;
  })
  

  onMount(async () => {

    await get_current_day();
  });


  async function get_current_day() {

    const res = await fetch('/current_day', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json'
        },
    })
    
    let json_res = await res.json();

    console.log("current_day_id: " + json_res.id)

    current_day.id = json_res.id;

    const res2 = await fetch('/get_day', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
      body: JSON.stringify({
          id: current_day.id,
        }),
    })

    let json_res2 = await res2.json();

    current_day.date = json_res2.datum;
    current_day.week_day = json_res2.week_day;
  }

  async function previous_day() {

    if (current_day.id == 1) {
      return
    }
    

    current_day.id = current_day.id - 1;

    const res2 = await fetch('/get_day', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
      body: JSON.stringify({
          id: current_day.id,
        }),
    })

    let json_res2 = await res2.json();

    current_day.date = json_res2.datum;
    current_day.week_day = json_res2.week_day;
  }

  async function next_day() {

    current_day.id = current_day.id + 1;



    const res2 = await fetch('/get_day', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
      body: JSON.stringify({
          id: current_day.id,
        }),
    })

    if(res2.status == 500) 
    {
      const res3 = await fetch('/add_day', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          id: current_day.id - 1,
        }),
      })
      
      const res4 = await fetch('/get_day', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
      body: JSON.stringify({
          id: current_day.id,
        }),
      })

      
      let json_res4 = await res4.json();

      current_day.date = json_res4.datum;
      current_day.week_day = json_res4.week_day;


    }
    else if (res2.status == 200) {
      let json_res2 = await res2.json();

      current_day.date = json_res2.datum;
      current_day.week_day = json_res2.week_day;
    }

  }
  

</script>


<section>
  <div class="plan-container">
    <h2 class="plan-heading">Vertretungsplan</h2>

    <div class="button-container">
      <button class="active" on:click={previous_day}>&lt;</button>
      <button class="not-active" on:click={get_current_day}>{current_day.week_day}, der {current_day.date}</button>
      <button class="active" on:click={next_day}>&gt;</button>
    </div>

    <div class="info-container">
      <h3 class="info-head">Infos</h3>

      <div class="infos">
        <textarea bind:value={info_text} cols="30" rows="10" class="info_textarea"></textarea> 
      </div>

    </div>

    <AdminTabelle day_id={current_day.id} />


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

    padding-top: 6px;
    padding-bottom: 6px;


    width: 30px;

    background-color: var(--accent);
  

    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: var(--secondary);

    border: none;
    border-radius: 10px;


     cursor: pointer;

  }

  .not-active {

    padding-top: 9px;
    padding-bottom: 9px;
    padding-left: 20px;
    padding-right: 20px;

    margin-left: 10px;
    margin-right: 10px;

    background-color: var(--secondary);

    /*width: 110px;*/
    
    font-family: 'Rubik', sans-serif;

    font-size: 16px;

    color: var(--accent);

    border: solid;
    border-color: var(--accent);
    border-width: thin;
    border-radius: 10px;

    
    cursor: pointer;
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
    
    margin-top: 2vh;
    margin-left: 7vh;
    margin-right: 7vh;
  }

  .info_textarea {

    font-family: 'rubik', sans-serif;


    width: 100%;

    height: 25vh;

    border: 1px solid black;

    border-radius: 5px;

  }
</style>
