import { useState } from "react";
import type { MenuProps } from 'antd';
import { Layout, Menu, theme } from 'antd';
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const { Header, Content, Footer, Sider } = Layout;
const items: MenuProps['items'] = [{ key: '1', label: 'Home' }, { key: '2', label: 'About' }]

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const {
    token: { colorBgContainer },
  } = theme.useToken();

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <Layout hasSider>
      <Sider
        style={{
          overflow: 'auto',
          height: '100vh',
          position: 'fixed',
          left: 0,
          top: 0,
          bottom: 0,
        }}
      >
        <Menu theme="dark" mode="inline" defaultSelectedKeys={['1']} items={items} />
      </Sider>
      <Layout className="site-layout" style={{ marginLeft: 200 }}>
        <Content style={{ margin: '24px 16px 0', overflow: 'initial' }}>
          <div className="container">
            <h1>Welcome to Tauri!</h1>

            <div className="row">
              <a href="https://vitejs.dev" target="_blank" rel="noopener">
                <img src="/vite.svg" className="logo vite" alt="Vite logo" />
              </a>
              <a href="https://tauri.app" target="_blank" rel="noopener">
                <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
              </a>
              <a href="https://reactjs.org" target="_blank" rel="noopener">
                <img src={reactLogo} className="logo react" alt="React logo" />
              </a>
            </div>

            <p>Click on the Tauri, Vite, and React logos to learn more.</p>

            <div className="row">
              <form
                onSubmit={(e) => {
                  e.preventDefault();
                  greet();
                }}
              >
                <input
                  id="greet-input"
                  onChange={(e) => setName(e.currentTarget.value)}
                  placeholder="Enter a name..."
                />
                <button type="submit">Greet</button>
              </form>
            </div>
            <p>{greetMsg}</p>
          </div>
        </Content>
      </Layout>
    </Layout>
  );
}

export default App;
