// C++11
#include <cstdlib>
#include <iostream>
#include <string>
#include <vector>
#include <queue>
#include <tuple>

using namespace std;

enum Direction
{
	GO_UP = 'N',
	GO_DOWN = 'S',
	GO_LEFT = 'W',
	GO_RIGHT = 'E',
	STAY = '0'
};

enum StartPosition
{
	TOP_LEFT = 0,
	TOP_RIGHT = 1,
	BOTTOM_RIGHT = 2,
	BOTTOM_LEFT = 3
};

enum KnightStatus
{
	GATHERING = 2147483647,
	EXITING = (-2147483647 - 1)
	// Positive: rescuing princess (0 ~ P-1)
	// Negative: beating monster (-1 ~ -M)
};

template <typename T> inline T** create_array(const size_t row, const size_t col)
{
	size_t size = sizeof(T);
	size_t point_size = sizeof(T*);
	T **arr = (T **)malloc(point_size * row + size * row * col);
	if (arr != NULL)
	{
		T *head = (T*)((size_t)arr + point_size * row);
		for (size_t i = 0; i < row; ++i)
		{
			arr[i] = (T*)((size_t)head + i * col * size);
			for (size_t j = 0; j < col; ++j)
				new (&arr[i][j]) T;
		}
	}
	return (T**)arr;
}

struct Coordinate
{
	size_t row, column;
};

constexpr double rescue_threshold = 0.5;

class PrincessesAndMonsters {
public:
	typedef tuple<size_t, double> target_priority;

	// Currently, I assume that princesses and monsters are still
	// TODO: create groups of knights to behave together (use array pointer) in case that monster is quite many comparing to the size of dungeon
	string initialize(const int S, const vector<int> princesses, const vector<int> monsters, const int K)
	{
		// Copy arguments and initialize variables
		size = S; num_knight = K; max_turn = S * S * S;
		Coordinate temp;
		princess_origin.reserve(princesses.size() / 2);
		for (auto p = princesses.begin(); p != princesses.end(); p += 2)
		{
			temp.row = *p; temp.column = *(p + 1);
			princess_origin.push_back(temp);
		}
		monster_origin.reserve(monsters.size() / 2);
		for (auto p = monsters.begin(); p != monsters.end(); p += 2)
		{
			temp.row = *p; temp.column = *(p + 1);
			monster_origin.push_back(temp);
		}
		knight_status.resize(K);
		knight_origin.reserve(K);
		num_princess = princess_origin.size();
		num_monsters = monster_origin.size();
		center.column = size / 2; center.row = size / 2;
#ifdef DEBUG
		cerr << num_princess << " Princess, " << num_monsters << " Monsters, " << K << " Knights" << endl;
#endif // DEBUG

		// Calculate princess to monsters distance
		int** PMdistance = create_array<int>(num_princess, num_monsters);
		for (size_t i = 0; i < num_princess; i++)
			for (size_t j = 0; j < num_monsters; j++)
				PMdistance[i][j] = manhattan_distance(princess_origin[i], monster_origin[j]);

		// Calculate princess priority
		for (size_t i = 0; i < num_princess; i++)
		{
			// if a monster is closer to princess, then the princess priority is higher
			// if a princess is farther to exit, then the princess priority is higher
			double score = 0;
			for (size_t j = 0; j < num_monsters; j++)
				score += PMdistance[i][j];
			score = nearest_exit_distance(princess_origin[i]) / (score / num_monsters);
			princess_queue.push(tie(i, score));
#ifdef DEBUG
			cerr << "Princess [" << i << "] Priority: " << score << endl;
#endif // DEBUG
		}

		// Assign princess to knights
		vector<char> result_array;
		int counter = 0;
		while (!princess_queue.empty() && counter < K)
		{
			size_t princess_target = get<0>(princess_queue.top());
			StartPosition start = nearest_exit(princess_origin[princess_target]);
			knight_origin.push_back(exit_coordinate(start));
			knight_status[counter++] = princess_target;
			result_array.push_back(start + '0');

			double new_score = get<1>(princess_queue.top()) - rescue_threshold;
			princess_queue_buffer.push(tie(princess_target, new_score));
			princess_queue.pop();
		}
		// Assign more than one knights to a princess
		if (counter < K) 
		{
			all_rescuing = true; // every princess has be assigned at least one knight
			while (!princess_queue_buffer.empty() && counter < K)
			{
				swap(princess_queue, princess_queue_buffer);
				while (!princess_queue.empty() && counter < K)
				{
					if (get<1>(princess_queue.top()) < rescue_threshold)
					{
						princess_queue.pop();
						continue;
					}

					// same as above {
					size_t princess_target = get<0>(princess_queue.top());
					StartPosition start = nearest_exit(princess_origin[princess_target]);
					knight_origin.push_back(exit_coordinate(start));
					knight_status[counter++] = princess_target;
					result_array.push_back(start + '0');

					double new_score = get<1>(princess_queue.top()) - rescue_threshold;
					princess_queue_buffer.push(tie(princess_target, new_score));
					princess_queue.pop();
					// }
				}
			}
		}
		// Assign remaining knights to monsters
		while (counter < K)
		{
			StartPosition start = nearest_exit(monster_origin[0]);
			knight_origin.push_back(exit_coordinate(start));
			knight_status[counter++] = -1;
			result_array.push_back(start + '0');
		}

		free((void**)PMdistance);
		result_array.push_back(0);
		string result(&result_array[0]);
#ifdef DEBUG
		cerr << "Knight Assignment: " << endl;
		for (auto status : knight_status)
			cerr << status << " ";
		cerr << endl;
#endif // DEBUG
		return result;
    }
    
	// TODO: Add more time for staying, like 1/10 of max turns
	// TODO: Add considering for princess count. if there is many princess remaining, then wander and wait for them
    string move(const vector<int> status, const int P, const int M, const int timeLeft) 
	{
		// Planning
		vector<char> result_array;
		for (int i = 0; i < num_knight; i++)
		{
			if (status[i] < 0)
			{
				result_array.push_back(STAY);
				continue; // skip invalid knights
			}
#ifdef DEBUG_VERBOSE
			cerr << "Knight " << i << ": ";
#endif // DEBUG_VERBOSE
			if (nearest_exit_distance(knight_origin[i]) > (max_turn - turn + 5)) // 2 is the tolerance
				knight_status[i] = EXITING; // exit immediately

			if (knight_status[i] == GATHERING)
			{
				// move to center
				Direction move = move_to(knight_origin[i], center);
				if (move == STAY)
				{
					bool waiting = false; // wait to exit the dungeon together
					for (size_t j = 0; j < num_knight && j != i && status[j] > 0; j++)
					{
						//  if (select_exit(j) != select_exit(i)) continue;
						if (knight_status[j] == GATHERING && manhattan_distance(knight_origin[j], center) > 0)
							waiting = true;
					}

					if (waiting)
					{
#ifdef DEBUG_VERBOSE
						cerr << "Waiting for gathering.." << endl;
#endif // DEBUG_VERBOSE
						result_array.push_back(STAY);
					}
					else
					{
						knight_status[i] = EXITING;
						i--; // rerun this loop
					}
				}
				else
				{
#ifdef DEBUG_VERBOSE
					cerr << "Gathering.." << endl;
#endif // DEBUG_VERBOSE
					apply_move(i, move);
					result_array.push_back(move);
				}
			}
			else if (knight_status[i] == EXITING)
			{
#ifdef DEBUG_VERBOSE
				cerr << "Exiting.." << endl;
#endif // DEBUG_VERBOSE
				Direction move = move_to(knight_origin[i], exit_coordinate(nearest_exit(knight_origin[i])));
				apply_move(i, move);
				result_array.push_back(move);
			}
			else if (knight_status[i] >= 0)
			{
				// move to princess
				Direction move = move_to(knight_origin[i], princess_origin[knight_status[i]]);
#ifdef DEBUG_VERBOSE
				cerr << "Rescuing princess " << knight_status[i] << "/" << num_princess << endl;
#endif // DEBUG_VERBOSE
				if (move == STAY)
				{
					// TODO: aftering get to the position of princess, wandering to check neighbour
					if (princess_queue.empty() || get<1>(princess_queue.top()) < rescue_threshold)
						knight_status[i] = GATHERING;
					else
					{
						knight_status[i] = get<0>(princess_queue.top());
						double new_score = get<1>(princess_queue.top()) - rescue_threshold; // TODO: reduce priority or discard directly?
						princess_queue.pop();
						princess_queue.push(tie(knight_status[i], new_score));
					}
					i--; // rerun this loop
				}
				else
				{
					apply_move(i, move);
					result_array.push_back(move);
				}
			}
			else
			{
				// move to monster
				int current_monster = -1 - knight_status[i];
				Direction move = move_to(knight_origin[i], monster_origin[current_monster]);
#ifdef DEBUG_VERBOSE
				cerr << "Killing monster " << current_monster << "/" << num_monsters << endl;
#endif // DEBUG_VERBOSE
				if (move == STAY)
				{
					if (current_monster >= num_monsters - 1)
						knight_status[i] = GATHERING;
					else
					{
						// skip monster at the exit
						do { current_monster++; } while (nearest_exit_distance(monster_origin[current_monster]) < 2); 
						// make all remaining knights hunting for the next monster
						for (int j = 0; j < num_knight && j != i && status[j] > 0; j++)
							if (knight_status[j] == knight_status[i])
								knight_status[j] = -current_monster - 1;
						knight_status[i] = -current_monster - 1;
					}
					i--; // rerun this loop
				}
				else
				{
					apply_move(i, move);
					result_array.push_back(move);
				}
			}
		}

		turn++;
		result_array.push_back(0);
		string result(&result_array[0]);
#ifdef DEBUG
		cerr << "Turn " << turn << ": " << result << endl;
#endif // DEBUG
		return result;
    }

private:
	size_t size, max_turn, turn = 0;
	Coordinate center;
	size_t num_princess, num_monsters, num_knight;
	vector<Coordinate> princess_origin, monster_origin, knight_origin;
	vector<int> knight_status;
	priority_queue<target_priority> princess_queue, princess_queue_buffer;
	bool all_rescuing = false;

	inline int nearest_exit_distance(const Coordinate &target)
	{
		switch (nearest_exit(target))
		{
		case TOP_LEFT:
			return target.row + target.column;
		case TOP_RIGHT:
			return target.row + size - target.column - 1;
		case BOTTOM_LEFT:
			return size - target.row + target.column - 1;
		case BOTTOM_RIGHT:
			return size - target.row + size - target.column - 2;
		}
	}

	inline Coordinate exit_coordinate(StartPosition position)
	{
		Coordinate result;
		switch (position)
		{
		case TOP_LEFT:
			result.row = 0; result.column = 0;
			break;
		case TOP_RIGHT:
			result.row = 0; result.column = size - 1;
			break;
		case BOTTOM_LEFT:
			result.row = size - 1; result.column = 0;
			break;
		case BOTTOM_RIGHT:
			result.row = size - 1; result.column = size - 1;
			break;
		}
		return result;
	}

	inline StartPosition nearest_exit(const Coordinate &target)
	{
		if (target.row < size / 2)
		{
			if (target.column < size / 2)
				return TOP_LEFT;
			else
				return TOP_RIGHT;
		}
		else
		{
			if (target.column < size / 2)
				return BOTTOM_LEFT;
			else
				return BOTTOM_RIGHT;
		}
	}

	inline int manhattan_distance(const Coordinate &a, const Coordinate &b)
	{
		return abs((int)a.row - (int)b.row) + abs((int)a.column - (int)b.column);
	}

	inline Direction move_to(const Coordinate &origin, const Coordinate &destination)
	{
		if (origin.column > destination.column)
			return GO_LEFT;
		else if (origin.column < destination.column)
			return GO_RIGHT;
		else if (origin.row > destination.row)
			return GO_UP;
		else if (origin.row < destination.row)
			return GO_DOWN;
		else return STAY;
	}

	inline void apply_move(size_t knight, Direction move)
	{
		switch (move)
		{
		case GO_UP:
			knight_origin[knight].row--;
			break;
		case GO_DOWN:
			knight_origin[knight].row++;
			break;
		case GO_LEFT:
			knight_origin[knight].column--;
			break;
		case GO_RIGHT:
			knight_origin[knight].column++;
			break;
		case STAY:
			break;
		}
	}

	inline StartPosition select_exit(size_t knight)
	{
		return (StartPosition)(knight % 4);
	}
};

bool operator<(PrincessesAndMonsters::target_priority a, PrincessesAndMonsters::target_priority b)
{
	return get<1>(a) < get<1>(b);
}
