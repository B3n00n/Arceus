import { useState, useMemo } from 'react';
import {
  Typography,
  Table,
  Input,
  Card,
  Tooltip,
  Flex,
  Button,
  Tag,
} from 'antd';
import {
  SearchOutlined,
  ReloadOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import { useSensors } from '../hooks/useSensors';
import type { TrackedSensor } from '../types';

dayjs.extend(relativeTime);

const { Title } = Typography;

export const SensorsPage = () => {
  const [searchText, setSearchText] = useState('');
  const { data: sensors = [], isLoading, refetch } = useSensors();

  const filteredSensors = useMemo(() => {
    return sensors.filter((sensor) => {
      const search = searchText.toLowerCase();
      return (
        sensor.serial_number.toLowerCase().includes(search) ||
        (sensor.mac_address?.toLowerCase().includes(search) ?? false) ||
        (sensor.firmware_version?.toLowerCase().includes(search) ?? false) ||
        (sensor.arcade_name?.toLowerCase().includes(search) ?? false)
      );
    });
  }, [sensors, searchText]);

  const columns: ColumnsType<TrackedSensor> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 70,
      sorter: (a, b) => a.id - b.id,
      render: (id: number) => (
        <span style={{ color: '#94a3b8', fontWeight: 500, fontSize: 13 }}>#{id}</span>
      ),
    },
    {
      title: 'Serial Number',
      dataIndex: 'serial_number',
      key: 'serial_number',
      sorter: (a, b) => a.serial_number.localeCompare(b.serial_number),
      render: (serial: string) => (
        <code
          style={{
            fontSize: 12,
            padding: '4px 8px',
            backgroundColor: '#0f172a',
            borderRadius: 4,
            color: '#06b6d4',
            border: '1px solid #334155',
            fontFamily: 'monospace',
          }}
        >
          {serial}
        </code>
      ),
    },
    {
      title: 'MAC Address',
      dataIndex: 'mac_address',
      key: 'mac_address',
      width: 180,
      render: (mac: string | null) =>
        mac ? (
          <code style={{ fontSize: 12, color: '#94a3b8', fontFamily: 'monospace' }}>
            {mac}
          </code>
        ) : (
          <span style={{ color: '#64748b', fontStyle: 'italic' }}>Unknown</span>
        ),
    },
    {
      title: 'Firmware',
      dataIndex: 'firmware_version',
      key: 'firmware_version',
      width: 140,
      render: (version: string | null) =>
        version ? (
          <Tag
            color="processing"
            style={{
              margin: 0,
              fontSize: 12,
              padding: '4px 10px',
              fontWeight: 600,
              fontFamily: 'monospace',
              borderRadius: 6,
            }}
          >
            {version}
          </Tag>
        ) : (
          <span style={{ color: '#64748b', fontStyle: 'italic' }}>Unknown</span>
        ),
      sorter: (a, b) =>
        (a.firmware_version ?? '').localeCompare(b.firmware_version ?? ''),
    },
    {
      title: 'Arcade',
      dataIndex: 'arcade_name',
      key: 'arcade_name',
      width: 200,
      render: (name: string | null) =>
        name ? (
          <span style={{ fontWeight: 500 }}>{name}</span>
        ) : (
          <span style={{ color: '#64748b', fontStyle: 'italic' }}>Unassigned</span>
        ),
      sorter: (a, b) =>
        (a.arcade_name ?? '').localeCompare(b.arcade_name ?? ''),
    },
    {
      title: 'Last Updated',
      dataIndex: 'updated_at',
      key: 'updated_at',
      width: 150,
      render: (updated: string) => (
        <Tooltip title={dayjs(updated).format('YYYY-MM-DD HH:mm:ss')}>
          {dayjs(updated).fromNow()}
        </Tooltip>
      ),
      sorter: (a, b) => dayjs(a.updated_at).unix() - dayjs(b.updated_at).unix(),
      defaultSortOrder: 'descend',
    },
    {
      title: 'Created',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 150,
      render: (created: string) => (
        <Tooltip title={dayjs(created).format('YYYY-MM-DD HH:mm:ss')}>
          {dayjs(created).fromNow()}
        </Tooltip>
      ),
      sorter: (a, b) => dayjs(a.created_at).unix() - dayjs(b.created_at).unix(),
    },
  ];

  return (
    <div style={{ padding: '8px 0' }}>
      <Flex justify="space-between" align="center" style={{ marginBottom: 24 }} wrap="wrap" gap={16}>
        <div>
          <Title level={2} style={{ margin: 0, fontSize: 28, fontWeight: 600 }}>
            Sensors
          </Title>
          <div style={{ marginTop: 8, color: '#94a3b8', fontSize: 14 }}>
            {filteredSensors.length} sensor{filteredSensors.length !== 1 ? 's' : ''} tracked
          </div>
        </div>
      </Flex>

      <Card
        style={{
          borderRadius: 12,
          boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.2), 0 2px 4px -2px rgb(0 0 0 / 0.2)',
        }}
      >
        <Flex gap={12} style={{ marginBottom: 20 }} wrap="wrap" align="center">
          <Input
            placeholder="Search by serial, MAC, firmware, or arcade..."
            prefix={<SearchOutlined style={{ color: '#64748b' }} />}
            allowClear
            style={{ maxWidth: 400, flex: 1 }}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            size="large"
          />
          <Tooltip title="Refresh Data">
            <Button
              icon={<ReloadOutlined />}
              onClick={() => refetch()}
              loading={isLoading}
              size="large"
            >
              Refresh
            </Button>
          </Tooltip>
        </Flex>

        <Table
          columns={columns}
          dataSource={filteredSensors}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 20,
            showTotal: (total) => `${total} sensor${total !== 1 ? 's' : ''} total`,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '50', '100'],
            style: { marginTop: 16 },
          }}
          scroll={{ x: 1000 }}
          style={{ borderRadius: 8, overflow: 'hidden' }}
        />
      </Card>
    </div>
  );
};
