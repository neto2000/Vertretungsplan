<script>

  import { onMount } from "svelte";


  let test = is_leap_year(2022)

  let year = 2024

  let month = 3

  let week_day = 3

  let active_day = 10



  let day_array = [[]];

  let week_dict = {
    1 : "January",
    2 : "Febuary",
    3 : "March",
    4 : "April",
    5 : "May",
    6 : "June",
    7 : "July",
    8 : "August",
    9 : "September",
    10 : "October",
    11 : "November",
    12 : "December",

  }
  

  onMount(() => {
    create_day_array()
  });



  function create_day_array() {

    console.log("on mount")


    let current_row = 0

    for (let i = 0; i < week_day; i++) {


      day_array[current_row].push("-")

      if (day_array[current_row].length >= 7) {

        day_array.push([])

        current_row++;
      }

    } 

    let month_length = 30

    for (let i = 1; i<= month_length; i++) {
      

      day_array[current_row].push(i)

      if (day_array[current_row].length >= 7) {

        day_array.push([])

        current_row++;
      }

    }
    
    while (day_array[current_row].length < 7){


      day_array[current_row].push("-")

    } 

    console.log(day_array)

    day_array = day_array
  }

  function get_weekday_of_first(month, year) {


  }

  function is_leap_year(year) {

    if (year % 4 != 0) {

      return false

    }

    if (year % 100 == 0) {

      if (year % 400 == 0) {

        return true
      }
      else {

        return false
      }

    }
    else {
      return true
    }


  }



  function previous_month() {
    
    month--
    
    if (month < 1) {

      month = 12

      year--
    } 
  }
  
  function next_month() {
  
    month++

    if (month > 12) {

      month = 1
      
      year++
    }

  }

</script>

<div class="date-container">

  <div class="button-container">
    <button class="active" on:click={previous_month}>&lt;</button>
    <button class="not-active">{week_dict[month]}</button>
    <button class="active" on:click={next_month}>&gt;</button>
  </div>

  <table class="table">
    <tr>
      <th class="table-head">Mo</th>
      <th class="table-head">Di</th>
      <th class="table-head">Mi</th>
      <th class="table-head">Do</th>
      <th class="table-head">Fr</th>
      <th class="table-head">Sa</th>
      <th class="table-head">So</th>
    </tr>

    {#each day_array as row}

      <tr>

        {#each row as item}
          <td>

            {#if item == active_day}
              <button class="date-active-button" on:click={() => {if (item != "-") active_day = item}}>{item}</button>
            {:else}
              <button class="date-button" on:click={() => {if (item != "-") active_day = item}}>{item}</button>
            {/if}            

            
          </td>
        {/each}
        
      </tr>

    {/each}      

  </table>

  <input bind:value={year} class="input" />

</div>


<style>

  .date-button {

    width: 35px;
    height: 35px;

    background-color: white;
    
    border-radius: 8px;
    border-style: solid;
    border-width: 1px;
    border-color: lightgray;

    cursor: pointer;
  }

  .date-active-button {

    width: 35px;
    height: 35px;

    background-color: var(--accent);
    color: white;

    border-radius: 8px;
    border-style: solid;
    border-width: 1px;
    border-color: var(--accent);

    cursor: pointer;
  }


  .table-head {
    
    padding-right: 5px;
    padding-left: 5px;

  }
    
  .date-container {

    position: absolute;

    left: 50%;

    transform: translateX(-50%);


    z-index: 10;

    text-align: center;
    

    border: none;

    border-radius: 10px;

    box-shadow: 0px 4px 4px 2px rgba(0, 0, 0, 0.2);

    background-color: white;


    margin-top: 22vh;

  }


  .button-container {

    margin-top: 2vh;   
  
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

    padding-top: 6px;
    padding-bottom: 6px;
    padding-left: 10px;
    padding-right: 10px;

    margin-left: 10px;
    margin-right: 10px;

    background-color: white;

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
  

  .table {
    border-collapse:separate;

    border-spacing: 0px;

    border-radius: 10px;

    border: solid;

    border-width: 1px;



    overflow: hidden;



    margin-top: 2vh;
    margin-bottom: 2vh;

    margin-left: 2vw;
    margin-right: 2vw;
    
  }

  .input {

    width: 5vw;

    margin-bottom: 2vh;

    padding-top: 0.5vh;
    padding-bottom: 0.5vh;


    border-radius: 8px;
    border-style: solid;
    border-width: 1px;
    border-color: black;


    text-align: center;

  }
 
</style>
