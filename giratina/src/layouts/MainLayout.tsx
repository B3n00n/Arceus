import { useState } from 'react';
import { Layout, Menu, theme } from 'antd';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import {
  DesktopOutlined,
  AppstoreOutlined,
  TagsOutlined,
  LinkOutlined,
  AndroidOutlined,
} from '@ant-design/icons';

const { Header, Content, Sider } = Layout;

export const MainLayout = () => {
  const [collapsed, setCollapsed] = useState(false);
  const navigate = useNavigate();
  const location = useLocation();
  const {
    token: { colorBgContainer },
  } = theme.useToken();

  const menuItems = [
    {
      key: '/arcades',
      icon: <DesktopOutlined />,
      label: 'Arcades',
    },
    {
      key: '/games',
      icon: <AppstoreOutlined />,
      label: 'Games',
    },
    {
      key: '/versions',
      icon: <TagsOutlined />,
      label: 'Game Versions',
    },
    {
      key: '/assignments',
      icon: <LinkOutlined />,
      label: 'Assignments',
    },
    {
      key: '/snorlax',
      icon: <AndroidOutlined />,
      label: 'Snorlax Versions',
    },
  ];

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider collapsible collapsed={collapsed} onCollapse={setCollapsed}>
        <div
          style={{
            height: 64,
            margin: 16,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontSize: collapsed ? 24 : 28,
            fontWeight: 'bold',
            background: 'linear-gradient(135deg, #dc2626 0%, #ea580c 50%, #f59e0b 100%)',
            WebkitBackgroundClip: 'text',
            WebkitTextFillColor: 'transparent',
            letterSpacing: collapsed ? 0 : 2,
          }}
        >
          {collapsed ? 'G' : 'GIRATINA'}
        </div>
        <Menu
          theme="dark"
          selectedKeys={[location.pathname]}
          mode="inline"
          items={menuItems}
          onClick={({ key }) => navigate(key)}
        />
      </Sider>
      <Layout>
        <Header
          style={{
            padding: '0 24px',
            background: colorBgContainer,
            display: 'flex',
            alignItems: 'center',
            borderBottom: '1px solid #1f1f1f',
          }}
        >
          <h2 style={{ margin: 0, fontWeight: 600, color: '#e5e5e5' }}>
            B3n00n The Almighty
          </h2>
        </Header>
        <Content style={{ margin: '24px 16px', overflow: 'auto' }}>
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
};
