select path, count(1) from entries
where fileExtension = 'GR2'
group by path
order by count(1) DESC