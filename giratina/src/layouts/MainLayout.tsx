import { useState, useEffect } from 'react';
import { Layout, Menu, theme, Button } from 'antd';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import {
  DesktopOutlined,
  AppstoreOutlined,
  TagsOutlined,
  LinkOutlined,
  AndroidOutlined,
  LeftOutlined,
  RightOutlined,
  MenuOutlined,
} from '@ant-design/icons';

const { Header, Content, Sider } = Layout;

export const MainLayout = () => {
  const [collapsed, setCollapsed] = useState(false);
  const [isMobile, setIsMobile] = useState(false);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const navigate = useNavigate();
  const location = useLocation();
  const {
    token: { colorBgContainer },
  } = theme.useToken();

  // Detect mobile screen size
  useEffect(() => {
    const checkMobile = () => {
      setIsMobile(window.innerWidth <= 768);
      if (window.innerWidth > 768) {
        setMobileMenuOpen(false);
      }
    };

    checkMobile();
    window.addEventListener('resize', checkMobile);
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

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

  const handleMenuClick = (key: string) => {
    navigate(key);
    if (isMobile) {
      setMobileMenuOpen(false);
    }
  };

  return (
    <Layout style={{ minHeight: '100vh' }}>
      {/* Mobile overlay backdrop */}
      {isMobile && mobileMenuOpen && (
        <div
          onClick={() => setMobileMenuOpen(false)}
          style={{
            position: 'fixed',
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            backgroundColor: 'rgba(0, 0, 0, 0.7)',
            zIndex: 999,
          }}
        />
      )}

      <Sider
        collapsed={isMobile ? false : collapsed}
        trigger={null}
        width={220}
        style={{
          position: isMobile ? 'fixed' : 'relative',
          left: isMobile && !mobileMenuOpen ? '-220px' : '0',
          top: 0,
          bottom: 0,
          zIndex: 1000,
          transition: 'left 0.3s ease',
        }}
      >
        <div
          style={{
            height: 56,
            margin: '16px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: collapsed && !isMobile ? 'center' : 'space-between',
            padding: collapsed && !isMobile ? '0' : '0 16px',
            borderRadius: 8,
            background: 'rgba(30, 58, 138, 0.1)',
            border: '1px solid rgba(30, 58, 138, 0.3)',
          }}
        >
          {!collapsed && (
            <div
              style={{
                fontSize: 14,
                fontWeight: 600,
                color: '#60a5fa',
                letterSpacing: 1,
              }}
            >
              Admin Portal
            </div>
          )}
          {!isMobile && (
            <Button
              type="text"
              icon={collapsed ? <RightOutlined /> : <LeftOutlined />}
              onClick={() => setCollapsed(!collapsed)}
              style={{
                color: '#60a5fa',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
              }}
            />
          )}
        </div>
        <Menu
          theme="dark"
          selectedKeys={[location.pathname]}
          mode="inline"
          items={menuItems}
          onClick={({ key }) => handleMenuClick(key)}
        />
        <div
          style={{
            position: 'absolute',
            bottom: 0,
            left: 0,
            right: 0,
            padding: collapsed && !isMobile ? '12px 4px' : '16px',
            textAlign: 'center',
            borderTop: '1px solid rgba(30, 58, 138, 0.3)',
            background: 'rgba(30, 58, 138, 0.05)',
          }}
        >
          {!collapsed || isMobile ? (
            <div
              style={{
                fontSize: 11,
                color: '#64748b',
                fontWeight: 500,
              }}
            >
              Giratina v1.0.0
            </div>
          ) : (
            <div
              style={{
                fontSize: 9,
                color: '#64748b',
                fontWeight: 500,
                lineHeight: 1.2,
              }}
            >
              v1.0
            </div>
          )}
        </div>
      </Sider>
      <Layout>
        <Header
          style={{
            padding: isMobile ? '0 16px' : '0 24px',
            background: colorBgContainer,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            borderBottom: '1px solid rgba(30, 58, 138, 0.3)',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
            {isMobile && (
              <Button
                type="text"
                icon={<MenuOutlined />}
                onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
                style={{
                  color: '#60a5fa',
                  fontSize: 18,
                }}
              />
            )}
            <h2
              style={{
                margin: 0,
                fontWeight: 600,
                color: '#60a5fa',
                fontSize: isMobile ? 16 : 20,
              }}
            >
              B3n00n The Almighty
            </h2>
          </div>
        </Header>
        <Content style={{ margin: '24px 16px', overflow: 'auto' }}>
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
};
