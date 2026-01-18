import { useState, useMemo } from 'react';
import {
  Typography,
  Button,
  Table,
  Input,
  Card,
  Popconfirm,
  Tooltip,
  Flex,
  Select,
  Tag,
  Progress,
  App,
} from 'antd';
import {
  PlusOutlined,
  DeleteOutlined,
  SearchOutlined,
  ReloadOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import {
  useGameVersions,
  useAllGameVersions,
  useDeleteGameVersion,
} from '../hooks/useGameVersions';
import { useGames } from '../hooks/useGames';
import { GameVersionModal } from '../components/GameVersionModal';
import { api } from '../services/api';
import type { GameVersion } from '../types';

dayjs.extend(relativeTime);

const { Title } = Typography;

interface GameVersionWithGame extends GameVersion {
  game_name?: string;
}

export const GameVersionsPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [searchText, setSearchText] = useState('');
  const [selectedGameFilter, setSelectedGameFilter] = useState<number | 'all'>('all');
  const [isUploading, setIsUploading] = useState(false);

  const { data: games = [] } = useGames();
  const gameIds = games.map(g => g.id);

  const { data: singleGameVersions = [], isLoading: isLoadingSingle, refetch: refetchSingle } =
    useGameVersions(typeof selectedGameFilter === 'number' ? selectedGameFilter : null);
  const { data: allVersions = [], isLoading: isLoadingAll, refetch: refetchAll } =
    useAllGameVersions(selectedGameFilter === 'all' ? gameIds : []);

  const versions = selectedGameFilter === 'all' ? allVersions : singleGameVersions;
  const isLoading = selectedGameFilter === 'all' ? isLoadingAll : isLoadingSingle;
  const refetch = selectedGameFilter === 'all' ? refetchAll : refetchSingle;

  const { message, modal } = App.useApp();
  const deleteMutation = useDeleteGameVersion();

  const enrichedVersions: GameVersionWithGame[] = useMemo(() => {
    return versions.map((version) => ({
      ...version,
      game_name: games.find((g) => g.id === version.game_id)?.name || 'Unknown',
    }));
  }, [versions, games]);

  const filteredVersions = useMemo(() => {
    return enrichedVersions.filter((version) =>
      version.version.toLowerCase().includes(searchText.toLowerCase()) ||
      version.gcs_path.toLowerCase().includes(searchText.toLowerCase()) ||
      version.game_name?.toLowerCase().includes(searchText.toLowerCase())
    );
  }, [enrichedVersions, searchText]);

  const handleCreate = () => {
    setModalOpen(true);
  };

  const handleDelete = async (version: GameVersion) => {
    await deleteMutation.mutateAsync({
      gameId: version.game_id,
      versionId: version.id,
    });
  };

  const handleModalSubmit = async (values: any) => {
    try {
      const { game_id, version, file } = values;

      if (!file) {
        message.error('Please select a ZIP file');
        return;
      }

      setIsUploading(true);
      setModalOpen(false);

      // Show progress modal
      const progressModal = modal.info({
        title: 'Uploading Game Version',
        content: (
          <div>
            <p>Uploading {file.name}...</p>
            <Progress percent={0} status="active" />
            <p style={{ marginTop: 16, color: '#666', fontSize: 12 }}>
              This may take several minutes for large files. Do not close this window.
            </p>
          </div>
        ),
        okButtonProps: { style: { display: 'none' } },
        closable: false,
        maskClosable: false,
      });

      try {
        await api.uploadGameVersion(game_id, version, file, (progress) => {
          progressModal.update({
            content: (
              <div>
                <p>Uploading {file.name}...</p>
                <Progress percent={progress} status="active" />
                <p style={{ marginTop: 16, color: '#666', fontSize: 12 }}>
                  {progress < 100
                    ? 'This may take several minutes for large files. Do not close this window.'
                    : 'Processing... The ZIP will be extracted automatically.'}
                </p>
              </div>
            ),
          });
        });

        progressModal.destroy();
        message.success(`Version ${version} uploaded successfully! Files will be extracted shortly.`);
        refetch();
      } catch (error: any) {
        progressModal.destroy();
        const errorMessage = error.response?.data?.error || error.message || 'Upload failed';
        message.error(errorMessage);
      } finally {
        setIsUploading(false);
      }
    } catch {
      // Error already handled above
    }
  };

  const handleModalCancel = () => {
    if (isUploading) {
      modal.confirm({
        title: 'Upload in progress',
        content: 'Are you sure you want to cancel? The upload will be interrupted.',
        onOk: () => {
          setModalOpen(false);
          setIsUploading(false);
        },
      });
    } else {
      setModalOpen(false);
    }
  };

  const columns: ColumnsType<GameVersionWithGame> = [
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
      title: 'Game',
      dataIndex: 'game_name',
      key: 'game_name',
      width: 200,
      render: (name: string) => (
        <Tag
          color="purple"
          style={{
            fontSize: 13,
            padding: '4px 12px',
            borderRadius: 6,
            fontWeight: 500,
          }}
        >
          {name}
        </Tag>
      ),
      sorter: (a, b) => (a.game_name || '').localeCompare(b.game_name || ''),
    },
    {
      title: 'Version',
      dataIndex: 'version',
      key: 'version',
      width: 120,
      sorter: (a, b) => a.version.localeCompare(b.version),
      render: (version: string) => (
        <Tag color="blue" style={{ fontSize: 13, padding: '4px 12px', borderRadius: 6, fontWeight: 600 }}>
          v{version}
        </Tag>
      ),
    },
    {
      title: 'GCS Path',
      dataIndex: 'gcs_path',
      key: 'gcs_path',
      width: 250,
      ellipsis: {
        showTitle: false,
      },
      render: (path: string) => (
        <Tooltip title={`Full path: ${path}`}>
          <code
            style={{
              fontSize: 12,
              padding: '4px 8px',
              backgroundColor: '#0f172a',
              borderRadius: 4,
              color: '#06b6d4',
              border: '1px solid #334155',
              display: 'inline-block',
              maxWidth: '100%',
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
            }}
          >
            {path}
          </code>
        </Tooltip>
      ),
    },
    {
      title: 'Release Date',
      dataIndex: 'release_date',
      key: 'release_date',
      width: 180,
      render: (date: string) => (
        <Tooltip title={dayjs(date).format('YYYY-MM-DD HH:mm:ss')}>
          {dayjs(date).fromNow()}
        </Tooltip>
      ),
      sorter: (a, b) => dayjs(a.release_date).unix() - dayjs(b.release_date).unix(),
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 80,
      fixed: 'right',
      align: 'center',
      render: (_, record) => (
        <Popconfirm
          title="Delete version?"
          description="This will also remove assignments using this version."
          onConfirm={() => handleDelete(record)}
          okText="Delete"
          okButtonProps={{ danger: true }}
          cancelText="Cancel"
        >
          <Tooltip title="Delete Version">
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
      ),
    },
  ];

  return (
    <div style={{ padding: '8px 0' }}>
      {/* Header Section */}
      <Flex justify="space-between" align="center" style={{ marginBottom: 24 }} wrap="wrap" gap={16}>
        <div>
          <Title level={2} style={{ margin: 0, fontSize: 28, fontWeight: 600 }}>
            Game Versions
          </Title>
          <div style={{ marginTop: 8, color: '#94a3b8', fontSize: 14 }}>
            {filteredVersions.length} version{filteredVersions.length !== 1 ? 's' : ''} total
          </div>
        </div>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
          style={{ minHeight: 42 }}
        >
          Upload Version
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
          <Select
            placeholder="Filter by game"
            style={{ minWidth: 200 }}
            value={selectedGameFilter}
            onChange={setSelectedGameFilter}
            options={[
              { label: 'All Games', value: 'all' },
              ...games.map((game) => ({
                label: game.name,
                value: game.id,
              })),
            ]}
            showSearch
            size="large"
            filterOption={(input, option) =>
              (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
            }
          />
          <Input
            placeholder="Search versions, paths..."
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

        {/* Data Table */}
        <Table
          columns={columns}
          dataSource={filteredVersions}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 10,
            showTotal: (total) => `${total} version${total !== 1 ? 's' : ''} total`,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '50', '100'],
            style: { marginTop: 16 },
          }}
          scroll={{ x: 1000 }}
          style={{ borderRadius: 8, overflow: 'hidden' }}
        />
      </Card>

      <GameVersionModal
        open={modalOpen}
        mode="create"
        version={undefined}
        onSubmit={handleModalSubmit}
        onCancel={handleModalCancel}
        loading={isUploading}
      />
    </div>
  );
};
