<script>

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


</script>

<div class="date-container">

  <h1>Hallo lol Datum</h1>

</div>


<style>
    
  .date-container {

    left: 50%;

    transform: translate(-50%,-50%);

    z-index: 10;

  }

 
</style>
