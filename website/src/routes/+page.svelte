<script>
</script>

<div class="page">
    <div class="header">
        <div class="header-lhs">
            <span class="nebula">nebula</span>
        </div>
        <div class="header-center">
            <button>Home</button>
            <button>Download</button>
            <button>Features</button>
            <button>About</button>
            <button>Blog</button>
            <button>Contact</button>
        </div>
        <div class="header-rhs">
            <a href="/login">
                <button class="login-button">
                    Log In
                </button>
            </a>
        </div>
    </div>
</div>

<style>
    * {
        box-sizing: border-box;
    }
    .page {
        display: flex;
        flex-direction: column;
        background-color: var(--background);
        width: 100vw;
        height: 100vh;
    }
    .header {
        display: flex;
        align-items: center;
        justify-content: space-around;
        height: 64px;
        font-family: Rubik, sans-serif;
    }
    .header button {
        cursor: pointer;
    }
    .header-center {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 24px;
        font-size: 21px;
    }
    .header-center button {
        border: none;
        background-color: transparent;
        transition: all 0.2s ease-in-out;
    }
    .header-center button:hover {
        color: var(--text);
    }
    .header-center button::after {
        content: '';
        display: block;
        height: 2px;
        width: 0;
        justify-self: center;
        border-radius: 32px;
        background: var(--text);
        transition: width 0.6s;
    }
    .header-center button:hover::after {
        width: 95%;
    }
    .header-rhs {
        display: flex;
        gap: 12px;
    }
    .header-rhs button {
        border-radius: 6px;
        padding: 10px 16px;
        font-size: 14px;
        font-weight: 500;
    }
    .header-rhs button:hover {
        box-shadow: 0 4px 4px rgba(0, 0, 0, 0.2);
    }
    .header-rhs a {
        text-decoration: none;
        color: inherit;
    }
    .login-button {
        border: none;
        background-image: linear-gradient(to right, var(--primary), var(--secondary));
        color: white;
        transition: box-shadow 0.4s ease-in-out;
    }
    .login-button:hover {
        box-shadow: inset 0 0 5px 3px rgba(0, 0, 0, 0.1), 0 4px 4px rgba(0, 0, 0, 0.2) !important;
    }
    .nebula {
        font-weight: 500;
        font-size: 18px;
    }
</style>