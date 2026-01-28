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
import { useChannels } from '../hooks/useChannels';
import { useGames } from '../hooks/useGames';
import { ArcadeModal } from '../components/ArcadeModal';
import { ChannelBadge } from '../components/ChannelBadge';
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
  const { data: channels = [] } = useChannels();
  const { data: games = [] } = useGames();
  const createMutation = useCreateArcade();
  const updateMutation = useUpdateArcade();
  const deleteMutation = useDeleteArcade();

  const getChannelName = (channelId: number) => {
    const channel = channels.find(c => c.id === channelId);
    return channel ? channel.name : 'Unknown';
  };

  const getGameName = (gameId: string) => {
    const game = games.find(g => g.id === parseInt(gameId));
    return game ? game.name : `Game #${gameId}`;
  };

  const filteredArcades = useMemo(() => {
    return arcades.filter((arcade) => {
      const matchesSearch =
        arcade.name.toLowerCase().includes(searchText.toLowerCase()) ||
        arcade.machine_id.toLowerCase().includes(searchText.toLowerCase());
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

  const expandedRowRender = (arcade: Arcade) => {
    if (!arcade.installed_games || Object.keys(arcade.installed_games).length === 0) {
      return (
        <div
          style={{
            padding: '24px',
            textAlign: 'center',
            backgroundColor: '#0f0f0f',
            borderTop: '1px solid #1e293b',
          }}
        >
          <span style={{ color: '#64748b', fontSize: 14 }}>
            No games installed on this arcade
          </span>
        </div>
      );
    }

    return (
      <div
        style={{
          padding: '20px 24px',
          backgroundColor: '#0f0f0f',
          borderTop: '1px solid #1e293b',
        }}
      >
        <div
          style={{
            marginBottom: 16,
            display: 'flex',
            alignItems: 'center',
            gap: 8,
          }}
        >
          <span style={{ color: '#94a3b8', fontSize: 12, fontWeight: 600, textTransform: 'uppercase', letterSpacing: '0.5px' }}>
            Installed Games
          </span>
          <Tag
            color="blue"
            style={{
              margin: 0,
              fontSize: 11,
              padding: '2px 8px',
              fontWeight: 600,
            }}
          >
            {Object.keys(arcade.installed_games).length}
          </Tag>
        </div>
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
            gap: 12,
          }}
        >
          {Object.entries(arcade.installed_games).map(([gameId, version]) => (
            <div
              key={gameId}
              style={{
                padding: '12px 16px',
                backgroundColor: '#1a1a1a',
                border: '1px solid #2a2a2a',
                borderRadius: 8,
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                transition: 'all 0.2s',
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = '#1e1e2e';
                e.currentTarget.style.borderColor = '#1e3a8a';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = '#1a1a1a';
                e.currentTarget.style.borderColor = '#2a2a2a';
              }}
            >
              <span style={{ color: '#e2e8f0', fontSize: 14, fontWeight: 500 }}>
                {getGameName(gameId)}
              </span>
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
            </div>
          ))}
        </div>
      </div>
    );
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
      title: 'Machine ID',
      dataIndex: 'machine_id',
      key: 'machine_id',
      render: (machineId: string) => (
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
          {machineId}
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
      title: 'Channel',
      dataIndex: 'channel_id',
      key: 'channel_id',
      width: 140,
      render: (channelId: number) => {
        const channelName = getChannelName(channelId);
        return <ChannelBadge channels={[{ id: channelId, name: channelName }]} />;
      },
      sorter: (a, b) => {
        const aName = getChannelName(a.channel_id);
        const bName = getChannelName(b.channel_id);
        return aName.localeCompare(bName);
      },
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
            description="This will permanently remove this arcade."
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
            placeholder="Search by name or machine ID..."
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
          expandable={{
            expandedRowRender,
            rowExpandable: (record) =>
              !!record.installed_games && Object.keys(record.installed_games).length > 0,
          }}
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
