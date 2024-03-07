<template>
  <button>
    <router-link to="/login">
      Click to Login
    </router-link>
  </button>
</template>

<script>
export default {
  name: 'button',
}
</script>

<style>
  button {
    width: 140px;
    height: 60px;
    color: white;
    background-color: #9b8a8a9d;
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
