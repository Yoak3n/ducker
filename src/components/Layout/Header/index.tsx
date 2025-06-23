import { Link } from "react-router-dom";

export default function Header() {
    return (
        <header data-tauri-drag-region>
            <nav data-tauri-drag-region>
                <ul className="nav-links">
                    <li>
                        <Link to="/">首页</Link>
                    </li>
                    <li>
                        <Link to="/about">关于</Link>
                    </li>
                    <li>
                        <Link to="/dashboard">面板</Link>
                    </li>
                </ul>
            </nav>
        </header>
    )
}