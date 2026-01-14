import { useState, useMemo } from 'react';
import {
  Typography,
  Button,
  Table,
  Space,
  Tag,
  Input,
  Select,
  Card,
  Popconfirm,
  Tooltip,
  Flex,
} from 'antd';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  SearchOutlined,
  ReloadOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import { useArcades, useCreateArcade, useUpdateArcade, useDeleteArcade } from '../hooks/useArcades';
import { ArcadeModal } from '../components/ArcadeModal';
import type { Arcade } from '../types';

dayjs.extend(relativeTime);

const { Title } = Typography;

const STATUS_COLORS: Record<string, string> = {
  active: 'success',
  inactive: 'default',
  maintenance: 'warning',
};

export const ArcadesPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<'create' | 'edit'>('create');
  const [selectedArcade, setSelectedArcade] = useState<Arcade | undefined>();
  const [searchText, setSearchText] = useState('');
  const [statusFilter, setStatusFilter] = useState<string | undefined>();

  const { data: arcades = [], isLoading, refetch } = useArcades();
  const createMutation = useCreateArcade();
  const updateMutation = useUpdateArcade();
  const deleteMutation = useDeleteArcade();

  const filteredArcades = useMemo(() => {
    return arcades.filter((arcade) => {
      const matchesSearch =
        arcade.name.toLowerCase().includes(searchText.toLowerCase()) ||
        arcade.mac_address.toLowerCase().includes(searchText.toLowerCase());
      const matchesStatus = !statusFilter || arcade.status === statusFilter;
      return matchesSearch && matchesStatus;
    });
  }, [arcades, searchText, statusFilter]);

  const handleCreate = () => {
    setModalMode('create');
    setSelectedArcade(undefined);
    setModalOpen(true);
  };

  const handleEdit = (arcade: Arcade) => {
    setModalMode('edit');
    setSelectedArcade(arcade);
    setModalOpen(true);
  };

  const handleDelete = async (id: number) => {
    await deleteMutation.mutateAsync(id);
  };

  const handleModalSubmit = async (values: any) => {
    try {
      if (modalMode === 'create') {
        await createMutation.mutateAsync(values);
      } else if (selectedArcade) {
        await updateMutation.mutateAsync({
          id: selectedArcade.id,
          data: values,
        });
      }
      setModalOpen(false);
      setSelectedArcade(undefined);
    } catch {
      // Error handling is done in the mutation hooks
    }
  };

  const handleModalCancel = () => {
    setModalOpen(false);
    setSelectedArcade(undefined);
  };

  const columns: ColumnsType<Arcade> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
      sorter: (a, b) => a.id - b.id,
      render: (id: number) => (
        <span style={{ color: '#94a3b8', fontWeight: 500, fontSize: 13 }}>#{id}</span>
      ),
    },
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
      sorter: (a, b) => a.name.localeCompare(b.name),
      render: (name: string) => (
        <span style={{ fontWeight: 600, fontSize: 14 }}>{name}</span>
      ),
    },
    {
      title: 'MAC Address',
      dataIndex: 'mac_address',
      key: 'mac_address',
      render: (mac: string) => (
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
          {mac}
        </code>
      ),
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      width: 130,
      render: (status: string) => (
        <Tag
          color={STATUS_COLORS[status] || 'default'}
          style={{
            fontSize: 12,
            padding: '4px 12px',
            borderRadius: 6,
            fontWeight: 600,
            textTransform: 'uppercase',
          }}
        >
          {status}
        </Tag>
      ),
      sorter: (a, b) => a.status.localeCompare(b.status),
    },
    {
      title: 'Last Seen',
      dataIndex: 'last_seen_at',
      key: 'last_seen_at',
      width: 150,
      render: (lastSeen: string | null) =>
        lastSeen ? (
          <Tooltip title={dayjs(lastSeen).format('YYYY-MM-DD HH:mm:ss')}>
            {dayjs(lastSeen).fromNow()}
          </Tooltip>
        ) : (
          <span style={{ color: '#666' }}>Never</span>
        ),
      sorter: (a, b) => {
        if (!a.last_seen_at) return 1;
        if (!b.last_seen_at) return -1;
        return dayjs(a.last_seen_at).unix() - dayjs(b.last_seen_at).unix();
      },
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
    {
      title: 'Actions',
      key: 'actions',
      width: 140,
      fixed: 'right',
      align: 'center',
      render: (_, record) => (
        <Space size={8}>
          <Tooltip title="Edit Arcade">
            <Button
              type="default"
              icon={<EditOutlined />}
              onClick={() => handleEdit(record)}
              size="middle"
              style={{
                borderRadius: 6,
                borderColor: '#475569',
              }}
            />
          </Tooltip>
          <Popconfirm
            title="Delete arcade?"
            description="This will remove all game assignments for this arcade."
            onConfirm={() => handleDelete(record.id)}
            okText="Delete"
            okButtonProps={{ danger: true }}
            cancelText="Cancel"
          >
            <Tooltip title="Delete Arcade">
              <Button
                danger
                icon={<DeleteOutlined />}
                size="middle"
                style={{
                  borderRadius: 6,
                }}
              />
            </Tooltip>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ padding: '8px 0' }}>
      {/* Header Section */}
      <Flex justify="space-between" align="center" style={{ marginBottom: 24 }} wrap="wrap" gap={16}>
        <div>
          <Title level={2} style={{ margin: 0, fontSize: 28, fontWeight: 600 }}>
            Arcades
          </Title>
          <div style={{ marginTop: 8, color: '#94a3b8', fontSize: 14 }}>
            {filteredArcades.length} arcade{filteredArcades.length !== 1 ? 's' : ''} total
          </div>
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
          style={{ minHeight: 42 }}
        >
          Create Arcade
        </Button>
      </Flex>

      {/* Main Content Card */}
      <Card
        style={{
          borderRadius: 12,
          boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.2), 0 2px 4px -2px rgb(0 0 0 / 0.2)',
        }}
      >
        {/* Filters and Search Bar */}
        <Flex gap={12} style={{ marginBottom: 20 }} wrap="wrap" align="center">
          <Input
            placeholder="Search by name or MAC address..."
            prefix={<SearchOutlined style={{ color: '#64748b' }} />}
            allowClear
            style={{ maxWidth: 400, flex: 1 }}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            size="large"
          />
          <Select
            placeholder="Filter by status"
            allowClear
            style={{ minWidth: 180 }}
            value={statusFilter}
            onChange={setStatusFilter}
            size="large"
            options={[
              { label: 'Active', value: 'active' },
              { label: 'Inactive', value: 'inactive' },
              { label: 'Maintenance', value: 'maintenance' },
            ]}
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

        {/* Data Table */}
        <Table
          columns={columns}
          dataSource={filteredArcades}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 10,
            showTotal: (total) => `${total} arcade${total !== 1 ? 's' : ''} total`,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '50', '100'],
            style: { marginTop: 16 },
          }}
          scroll={{ x: 1200 }}
          style={{ borderRadius: 8, overflow: 'hidden' }}
        />
      </Card>

      <ArcadeModal
        open={modalOpen}
        mode={modalMode}
        arcade={selectedArcade}
        onSubmit={handleModalSubmit}
        onCancel={handleModalCancel}
        loading={createMutation.isPending || updateMutation.isPending}
      />
    </div>
  );
};
