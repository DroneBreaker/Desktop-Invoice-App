<template>
  <button id="btn">
    <RouterLink to="/login">
      {{ title }}
    </RouterLink>
  </button>
</template>

<script setup>
/**
* Page Title
*/
  const title = "Click to Login"
</script>

<style>
  #btn {
    width: 150px;
    height: 60px;
    color: white;
    background-color: #329ea3;
    border-radius: 8px;
    cursor: pointer;
    font-size: large;
  }

  a{
    text-decoration: none;
    color: white;
  }

  a:hover {
    color: #24c8db;
  }
</style>
