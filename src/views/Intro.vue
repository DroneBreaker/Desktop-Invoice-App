<template>
    <div>
        <center>
            <h2>Welcome to Bukz Invoicing App</h2>
            <Button/>
        </center>
    </div>
</template>

<script>
import Button from '../components/Button.vue'

export default {
    name: 'intro',
    components: {
        Button
    }
}
</script>

<style>
h2 {
    font-weight: 400;
    font-size: 40px;
    margin-top: 25%;
}
</style>