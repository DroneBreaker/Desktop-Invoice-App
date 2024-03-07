<template>
    <div>
        <center>
            <h2 class="mt-[240px] text-4xl font-light mb-4">Welcome to Bukz Invoicing App</h2>
            <Button />
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
/* h2 {
    font-weight: 400;
    font-size: 40px;
    margin-top: 25%;
} */
</style>